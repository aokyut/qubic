use crate::board;
// use test::Bencher;

#[cfg(test)]
pub mod tests {
    use super::{board, board::_is_win_board};
    use std::time::{Duration, Instant};

    #[test]
    fn board_next() {
        let mut board = board::Board::new();
        board = board.next(0);
        assert_eq!(board.black, 0x0000000000000001);
        board = board.next(0);
        assert_eq!(board.white, 0x0000000000010000);
        board = board.next(0);
        assert_eq!(board.black, 0x0000000100000001);
        board = board.next(0);
        assert_eq!(board.white, 0x0001000000010000);
    }

    #[test]
    fn is_win_test() {
        assert!(_is_win_board(0xf000000000000000));
        assert!(_is_win_board(0x0100010001000100));
        assert!(_is_win_board(0x2222000000000000));
        assert!(_is_win_board(0x8421000000000000));
        assert!(_is_win_board(0x0000124800000000));
        assert!(!_is_win_board(0x0000000000000000));
        assert!(_is_win_board(0x0008000400020001));
        assert!(_is_win_board(0x8000040000200001));
        assert!(_is_win_board(0x2000020000200002));
        assert!(_is_win_board(0x0004004004004000));
        // assert!(_is_win_board(0x0000000000000000));
    }

    #[test]
    fn bench_minimaxs() {
        for depth in 1..=5 {
            let mut time_acum = 0;
            let mut step = 0;
            loop {
                let (_t, _s) = _bench_minimax(depth);
                time_acum += _t;
                step += _s;
                if time_acum > 5_000_000_000 {
                    break;
                }
            }
            println!(
                "[depth:{}] {} ns/times. -- acum:{}, times:{}",
                depth,
                time_acum / step,
                time_acum,
                step
            );
        }
    }

    #[test]
    fn bench_mcts() {
        for nums in vec![50, 100, 200, 400, 800, 1600] {
            let mut time_acum = 0;
            let mut step = 0;
            loop {
                let (_t, _s) = _bench_mcts(nums, 50);
                time_acum += _t;
                step += _s;
                if time_acum > 5_000_000_000 {
                    break;
                }
            }
            println!(
                "[search_n:{}] {}ns/search,  {} ns/times. -- acum:{}, times:{}",
                nums,
                time_acum / step / nums as u128,
                time_acum / step,
                time_acum,
                step
            );
        }
    }

    // #[test]
    fn _bench_minimax(depth: u8) -> (u128, u128) {
        let mut b = board::Board::new();
        let mut step = 0;
        let mut time_acum = 0;
        loop {
            time_acum += get_time_nanos(|| {
                b.minimax_action(depth);
            });
            step += 1;
            let action = board::get_random(&b);
            b = b.next(action);
            if b.is_win() || b.is_draw() {
                break;
            }
        }
        return (time_acum, step);
    }

    fn _bench_mcts(num: usize, expand_n: usize) -> (u128, u128) {
        let mut b = board::Board::new();
        let mut step = 0;
        let mut time_acum = 0;
        loop {
            time_acum += get_time_nanos(|| {
                board::mcts_action(&b, num, expand_n);
            });
            step += 1;
            let action = board::get_random(&b);
            b = b.next(action);
            if b.is_win() || b.is_draw() {
                break;
            }
        }

        return (time_acum, step);
    }

    fn get_time_nanos<F: Fn()>(f: F) -> u128 {
        let start = Instant::now();
        f();
        let end = start.elapsed();
        return end.as_nanos();
    }
}
