use crate::{config::EloConfig, outcomes::Outcomes, rating::EloRating};

/// Calculates the elo scores of two players based on their ratings and the outcome of the game.
///
/// Takes in two players, the outcome of the game and an [`EloConfig`].
///
/// The outcome of the match is in the perspective of `player_one`.
/// This means `Outcomes::WIN` is a win for `player_one` and `Outcomes::LOSS` is a win for `player_two`.
///
/// # Example
/// ```
/// use skillratings::{elo::elo, outcomes::Outcomes, rating::EloRating, config::EloConfig};
///
/// let player_one = EloRating { rating: 1000.0 };
/// let player_two = EloRating { rating: 1000.0 };
///
/// let outcome = Outcomes::WIN;
///
/// let config = EloConfig::new();
///
/// let (player_one_new, player_two_new) = elo(player_one, player_two, outcome, &config);
///
/// assert!((player_one_new.rating - 1016.0).abs() < f64::EPSILON);
/// assert!((player_two_new.rating - 984.0).abs() < f64::EPSILON);
/// ```
///
/// # More
/// [Wikipedia Article on the Elo system](https://en.wikipedia.org/wiki/Elo_rating_system).
#[must_use]
pub fn elo(
    player_one: EloRating,
    player_two: EloRating,
    outcome: Outcomes,
    config: &EloConfig,
) -> (EloRating, EloRating) {
    let (one_expected, two_expected) = expected_score(player_one, player_two);

    let o = match outcome {
        Outcomes::WIN => 1.0,
        Outcomes::LOSS => 0.0,
        Outcomes::DRAW => 0.5,
    };

    let one_new_elo = config.k.mul_add(o - one_expected, player_one.rating);
    let two_new_elo = config
        .k
        .mul_add((1.0 - o) - two_expected, player_two.rating);

    (
        EloRating {
            rating: one_new_elo,
        },
        EloRating {
            rating: two_new_elo,
        },
    )
}

/// Calculates a Elo Rating in a non-traditional way using a rating period,
/// for compatibility with the other algorithms.
///
/// Takes in a player and their results as a Vec of tuples containing the opponent and the outcome.
///
/// All of the outcomes are from the perspective of `player_one`.
/// This means `Outcomes::WIN` is a win for `player_one` and `Outcomes::LOSS` is a win for `player_two`.
///
/// # Example
/// ```
/// use skillratings::{elo::elo_rating_period, outcomes::Outcomes, rating::EloRating, config::EloConfig};
///
/// let player = EloRating::new();
///
/// let opponent1 = EloRating::new();
/// let opponent2 = EloRating::new();
/// let opponent3 = EloRating::new();
///
/// let new_player = elo_rating_period(
///     player,
///     &vec![
///         (opponent1, Outcomes::WIN),
///         (opponent2, Outcomes::WIN),
///         (opponent3, Outcomes::WIN),
///     ],
///     &EloConfig::new(),
/// );
///
/// assert!((new_player.rating.round() - 1046.0).abs() < f64::EPSILON);
/// ```
#[must_use]
pub fn elo_rating_period(
    player: EloRating,
    results: &Vec<(EloRating, Outcomes)>,
    config: &EloConfig,
) -> EloRating {
    let mut player = player;

    for (opponent, result) in results {
        (player, _) = elo(player, *opponent, *result, config);
    }

    player
}

/// Calculates the expected score of two players based on their elo rating.
/// Meant for usage in the elo function, but you can also use it to predict games yourself.
///
/// Takes in two elo scores and returns the expected score of each player.
/// A score of 1.0 means certain win, a score of 0.0 means certain loss, and a score of 0.5 is a draw.
///
/// # Example
/// ```
/// use skillratings::{elo::expected_score, rating::EloRating};
///
/// let player_one = EloRating { rating: 1320.0 };
/// let player_two = EloRating { rating: 1217.0 };
///
/// let (winner_exp, loser_exp) = expected_score(player_one, player_two);
///
/// assert!(((winner_exp * 100.0).round() - 64.0).abs() < f64::EPSILON);
/// assert!(((loser_exp * 100.0).round() - 36.0).abs() < f64::EPSILON);
/// ```
#[must_use]
pub fn expected_score(player_one: EloRating, player_two: EloRating) -> (f64, f64) {
    (
        1.0 / (1.0 + 10_f64.powf((player_two.rating - player_one.rating) / 400.0)),
        1.0 / (1.0 + 10_f64.powf((player_one.rating - player_two.rating) / 400.0)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elo() {
        let (winner_new_elo, loser_new_elo) = elo(
            EloRating { rating: 1000.0 },
            EloRating { rating: 1000.0 },
            Outcomes::WIN,
            &EloConfig::new(),
        );
        assert!((winner_new_elo.rating - 1016.0).abs() < f64::EPSILON);
        assert!((loser_new_elo.rating - 984.0).abs() < f64::EPSILON);

        let (winner_new_elo, loser_new_elo) = elo(
            EloRating { rating: 1000.0 },
            EloRating { rating: 1000.0 },
            Outcomes::LOSS,
            &EloConfig::new(),
        );
        assert!((winner_new_elo.rating - 984.0).abs() < f64::EPSILON);
        assert!((loser_new_elo.rating - 1016.0).abs() < f64::EPSILON);

        let (winner_new_elo, loser_new_elo) = elo(
            EloRating { rating: 1000.0 },
            EloRating { rating: 1000.0 },
            Outcomes::DRAW,
            &EloConfig::new(),
        );
        assert!((winner_new_elo.rating - 1000.0).abs() < f64::EPSILON);
        assert!((loser_new_elo.rating - 1000.0).abs() < f64::EPSILON);

        let (winner_new_elo, loser_new_elo) = elo(
            EloRating { rating: 500.0 },
            EloRating { rating: 1500.0 },
            Outcomes::WIN,
            &EloConfig::default(),
        );
        assert!((winner_new_elo.rating.round() - 532.0).abs() < f64::EPSILON);
        assert!((loser_new_elo.rating.round() - 1468.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_elo_rating_period() {
        let player = EloRating::new();

        let opponent1 = EloRating::new();
        let opponent2 = EloRating::new();
        let opponent3 = EloRating::new();

        let new_player = elo_rating_period(
            player,
            &vec![
                (opponent1, Outcomes::WIN),
                (opponent2, Outcomes::WIN),
                (opponent3, Outcomes::WIN),
            ],
            &EloConfig::new(),
        );

        assert!((new_player.rating.round() - 1046.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_expected_score() {
        let player_one = EloRating::new();
        let player_two = EloRating::default();

        let (winner_expected, loser_expected) = expected_score(player_one, player_two);

        assert!((winner_expected - 0.5).abs() < f64::EPSILON);
        assert!((loser_expected - 0.5).abs() < f64::EPSILON);

        let player_one = EloRating { rating: 2251.0 };
        let player_two = EloRating { rating: 1934.0 };

        let (winner_expected, loser_expected) = expected_score(player_one, player_two);

        assert!(((winner_expected * 100.0).round() - 86.0).abs() < f64::EPSILON);
        assert!(((loser_expected * 100.0).round() - 14.0).abs() < f64::EPSILON);

        assert!((winner_expected + loser_expected - 1.0).abs() < f64::EPSILON);
    }
}
