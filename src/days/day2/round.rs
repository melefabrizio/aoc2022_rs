use crate::days::day2::people::People;
use crate::days::day2::people::People::{Draw, Me, Oppo};
use crate::days::day2::Shape;
use crate::days::day2::Shape::{A, B, C};

fn shape_score(shape: Shape) -> i32 {
    match shape {
        A => 1,
        B => 2,
        C => 3,
    }
}

fn outcome_score(people: People) -> i32 {
    match people {
        Me => 6,
        Draw => 3,
        Oppo => 0,
    }
}

#[derive(Debug)]
pub(crate) struct Round {
    oppo: Shape,
    me: Option<Shape>,
    winner: Option<People>,
    pub(crate) points: Option<i32>,
}

impl Round {
    pub(crate) fn new(oppo: Shape, me: Option<Shape>, winner: Option<People>) -> Round {
        Round { oppo, me, winner, points: None }
    }
    pub(crate) fn calc_winner(&mut self) -> People {
        // A = rock, B = paper, C = scissors  for opponent
        // X = Rock, Y = Paper, Z = Scissors for me
        let mut me: Option<Shape> = None;
        if self.oppo == A {
            if self.winner == Some(Me) {
                me = Some(B);
            } else if self.winner == Some(Oppo) {
                me = Some(C);
            } else {
                me = Some(A);
            }
        } else if self.oppo == B {
            if self.winner == Some(Me) {
                me = Some(C);
            } else if self.winner == Some(Oppo) {
                me = Some(A);
            } else {
                me = Some(B);
            }
        } else if self.oppo == C {
            if self.winner == Some(Me) {
                me = Some(A);
            } else if self.winner == Some(Oppo) {
                me = Some(B);
            } else {
                me = Some(C);
            }
        }
        self.me = me;
        self.calc_score();
        return self.winner.unwrap();
    }
    fn calc_score(&mut self) -> i32 {
        let mut points = 0;
        points += shape_score(self.me.unwrap());
        points += outcome_score(self.winner.unwrap_or(Oppo));
        self.points = Some(points);
        return points;
    }
}
