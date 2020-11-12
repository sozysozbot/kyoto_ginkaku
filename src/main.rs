#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Piece {
    Kyo,
    To,
    Gin,
    Kaku,
    Kin,
    Kei,
    Hi,
    Fu,
    Ou,
}

impl Piece {
    fn conjugate(self) -> Piece {
        match self {
            Kyo => To,
            To => Kyo,
            Gin => Kaku,
            Kaku => Gin,
            Kin => Kei,
            Kei => Kin,
            Hi => Fu,
            Fu => Hi,
            Ou => Ou,
        }
    }
    fn serialize(self) -> &'static str {
        match self {
            Kyo => "香",
            To => "と",
            Gin => "銀",
            Kaku => "角",
            Kin => "金",
            Kei => "桂",
            Hi => "飛",
            Fu => "歩",
            Ou => "王",
        }
    }
}

fn print_board(a: &Board) {
    fn s(a: Option<(Piece, Side)>) -> String {
        match a {
            None => String::from("　　"),
            Some((p, s)) => format!("{}{}", p.serialize(), s.serialize()),
        }
    }
    let mut ans = String::new();

    ans += "-－－-－－-－－-－－-－－-\n";
    for i in 0..5 {
        ans += "|";
        for j in 0..5 {
            ans += &s(a[i][j]);
            ans += "|";
        }
        ans += "\n";
        ans += "-－－-－－-－－-－－-－－-\n";
    }

    print!("{}", ans);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    PointsUp,
    PointsDn,
}

impl Side {
    fn serialize(self) -> &'static str {
        match self {
            PointsUp => "↑",
            PointsDn => "↓",
        }
    }
}

use Piece::*;
use Side::*;

const INITIAL: Board = [
    [
        Some((Fu, PointsDn)),
        Some((Kin, PointsDn)),
        Some((Ou, PointsDn)),
        Some((Gin, PointsDn)),
        Some((To, PointsDn)),
    ],
    [None, None, None, None, None],
    [None, None, None, None, None],
    [None, None, None, None, None],
    [
        Some((To, PointsUp)),
        Some((Gin, PointsUp)),
        Some((Ou, PointsUp)),
        Some((Kin, PointsUp)),
        Some((Fu, PointsUp)),
    ],
];

type Board = [[Option<(Piece, Side)>; 5]; 5];

fn main() {
    print_board(&INITIAL)
}
