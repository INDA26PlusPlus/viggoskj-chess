#[cfg(test)]
mod tests {
    use crate::{
        check::{CheckState, get_check_state},
        game::Color,
        tests::tests::game_from_board_advanced,
    };

    #[test]
    fn check_mate_1() {
        let game = game_from_board_advanced(
            "
--------
--------
--------
--r----r
---K-r--
----r--r
--------    
--------",
            Color::White,
        );

        assert_eq!(get_check_state(&game), Some(CheckState::Checkmate));
    }

    #[test]
    fn stalemate_1() {
        let game = game_from_board_advanced(
            "
--------
--------
--------
--r----r
---K----
----r--r
--------    
--------",
            Color::White,
        );
    }

    #[test]
    fn check_1() {
        let game = game_from_board_advanced(
            "
--------
--------
--------
--------
---K---r
--------
--------    
--------",
            Color::White,
        );

        assert_eq!(get_check_state(&game), Some(CheckState::Check));
    }

    #[test]
    fn none_1() {
        let game = game_from_board_advanced(
            "
--------
--------
--------
--------
---K----
----n---
--------    
--------",
            Color::White,
        );

        assert_eq!(get_check_state(&game), None);
    }
}
