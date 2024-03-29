// cf. https://github.com/tanakh/simulated-annealing/blob/master/src/lib.rs
// ステップ数じゃなくて実行時間で終了を制御するように
// スコアが小さい方が良い

mod sa {
    use rand::prelude::*;
    use std::time::SystemTime;

    #[derive(Clone)]
    pub struct AnnealingOptions {
        pub time_limit: f64, // 焼きなまし実行時間
        pub limit_temp: f64, // 最低温度？
        pub silent: bool,    // デバッグログの表示有無
    }

    pub trait Annealer {
        type State: Clone;
        // 近傍への移動の表現
        type Move;

        fn init_state(&self, rng: &mut impl Rng) -> Self::State;
        fn start_temp(&self, init_score: f64) -> f64;

        // 強制終了する条件
        fn is_done(&self, _score: f64) -> bool {
            false
        }

        fn eval(&self, state: &Self::State) -> f64;

        fn neighbour(&self, state: &Self::State, rng: &mut impl Rng) -> Self::Move;

        // 近傍への移動の適用とアンドゥ
        fn apply(&self, state: &mut Self::State, mov: &Self::Move);
        fn unapply(&self, state: &mut Self::State, mov: &Self::Move);

        fn apply_and_eval(
            &self,
            state: &mut Self::State,
            mov: &Self::Move,
            _prev_score: f64,
        ) -> f64 {
            self.apply(state, mov);
            self.eval(state)
        }
    }

    pub fn annealing<A: Annealer>(
        annealer: &A,
        opt: &AnnealingOptions,
        seed: u64,
    ) -> (f64, <A as Annealer>::State) {
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut state = annealer.init_state(&mut rng);
        let mut cur_score = annealer.eval(&state);
        let mut best_score = cur_score;
        let mut best_ans = state.clone();

        macro_rules! progress {
            ($($arg:expr),*) => {
                if !opt.silent {
                    eprintln!($($arg),*);
                }
            };
        }

        progress!("Initial score: {}", cur_score);

        let t_max = annealer.start_temp(cur_score);
        let t_min = opt.limit_temp;

        let timer = SystemTime::now();
        let time_limit = opt.time_limit;

        let mut temp = t_max;
        let mut progress_ratio: f64;

        for i in 0.. {
            if i % 100 == 0 {
                progress_ratio = timer.elapsed().unwrap().as_secs_f64() / time_limit;
                if progress_ratio >= 1.0 {
                    break;
                }

                temp = t_max * (t_min / t_max).powf(progress_ratio);
            }

            let mov = annealer.neighbour(&state, &mut rng);
            let new_score = annealer.apply_and_eval(&mut state, &mov, cur_score);

            if new_score <= cur_score
                || rng.gen::<f64>() <= ((cur_score - new_score) as f64 / temp).exp()
            {
                cur_score = new_score;
                if cur_score < best_score {
                    if best_score - cur_score > 1e-6 {
                        progress!("Best: score = {:.3}, temp = {:.9}", cur_score, temp);
                    }
                    best_score = cur_score;
                    best_ans = state.clone();
                }
                if annealer.is_done(cur_score) {
                    break;
                }
            } else {
                annealer.unapply(&mut state, &mov);
            }
        }

        (best_score, best_ans)
    }
}
