#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Profession {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Captured {
    KyoTo,
    GinKaku,
    KinKei,
    HiFu,
}

impl Captured {
    fn serialize(self) -> &'static str {
        match self {
            KyoTo => "香と",
            GinKaku => "銀角",
            KinKei => "金桂",
            HiFu => "飛歩",
        }
    }
}

use Captured::*;

impl Profession {
    fn to_captured(self) -> Option<Captured> {
        match self {
            Kyo => Some(KyoTo),
            To => Some(KyoTo),
            Gin => Some(GinKaku),
            Kaku => Some(GinKaku),
            Kin => Some(KinKei),
            Kei => Some(KinKei),
            Hi => Some(HiFu),
            Fu => Some(HiFu),
            Ou => None,
        }
    }

    fn conjugate(self) -> Profession {
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

pub fn print_state(a: &State) {
    for p in &a.gote_hand {
        print!("{} ", p.serialize())
    }
    println!("");
    print_board(&a.b);
    for p in &a.sente_hand {
        print!("{} ", p.serialize())
    }
    println!("");
}

pub fn print_board(a: &Board) {
    fn s(a: Option<(Profession, Side)>) -> String {
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
pub enum Side {
    Sente,
    Gote,
}

impl std::ops::Not for Side {
    type Output = Side;
    fn not(self) -> Self {
        match self {
            Sente => Gote,
            Gote => Sente,
        }
    }
}

impl Side {
    fn serialize(self) -> &'static str {
        match self {
            Sente => "↑",
            Gote => "↓",
        }
    }
}

use Profession::*;
use Side::*;

pub const INITIAL: Board = [
    [
        Some((Fu, Gote)),
        Some((Kin, Gote)),
        Some((Ou, Gote)),
        Some((Gin, Gote)),
        Some((To, Gote)),
    ],
    [None, None, None, None, None],
    [None, None, None, None, None],
    [None, None, None, None, None],
    [
        Some((To, Sente)),
        Some((Gin, Sente)),
        Some((Ou, Sente)),
        Some((Kin, Sente)),
        Some((Fu, Sente)),
    ],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub b: Board,
    pub whose_turn: Side,
    pub sente_hand: Vec<Captured>,
    pub gote_hand: Vec<Captured>,
}

type Board = [[Option<(Profession, Side)>; 5]; 5];


type Coord = (Column, Row);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Row {
    Yi,
    Er,
    San,
    Si,
    Wu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Column {
    _1,
    _2,
    _3,
    _4,
    _5,
}

use Column::*;
use Row::*;


fn parse_coord(a: char, b: char) -> Option<Coord> {
    let col = match a {
        '1' => _1,
        '2' => _2,
        '3' => _3,
        '4' => _4,
        '5' => _5,
        _ => return None,
    };

    let row = match b {
        '一' => Yi,
        '二' => Er,
        '三' => San,
        '四' => Si,
        '五' => Wu,
        '1' => Yi,
        '2' => Er,
        '3' => San,
        '4' => Si,
        '5' => Wu,
        _ => return None,
    };

    Some((col, row))
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Movement {
    side: Side,
    dst: Coord,
    prof: Profession,
    src: Option<Coord>, /* If None, 打 */
}

pub fn M(s: &str) -> Movement {
    parse_movement(s).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateOrVictory {
    State(State),
    Victory(Side),
}

#[test]
fn it_works() {
    parse_movement("☗4四玉(35)").unwrap();
}


fn parse_movement(s: &str) -> Option<Movement> {
    let mut it = s.chars();

    let side = match it.next()? {
        '☗' => Sente,
        '☖' => Gote,
        '▲' => Sente,
        '△' => Gote,
        _ => return None,
    };

    let dst = parse_coord(it.next()?, it.next()?)?;

    let prof = match it.next()? {
        '香' => Kyo,
        'と' => To,
        '銀' => Gin,
        '角' => Kaku,
        '金' => Kin,
        '桂' => Kei,
        '飛' => Hi,
        '歩' => Fu,
        '王' => Ou,
        '玉' => Ou,
        _ => return None,
    };

    let src = match it.next()? {
        '打' => None,
        '(' => {
            let loc = parse_coord(it.next()?, it.next()?)?;
            if it.next()? != ')' {
                return None;
            }
            Some(loc)
        }
        _ => return None,
    };

    Some(Movement {
        side,
        dst,
        prof,
        src,
    })
}

fn foo(c: Coord) -> (usize, usize) {
    let (col, row) = c;
    let col = match col {
        _1 => 4,
        _2 => 3,
        _3 => 2,
        _4 => 1,
        _5 => 0,
    };
    let row = match row {
        Yi => 0,
        Er => 1,
        San => 2,
        Si => 3,
        Wu => 4,
    };
    (col, row)
}

fn get_mut(b: &mut Board, c: Coord) -> &mut Option<(Profession, Side)> {
    let (col, row) = foo(c);
    &mut b[row][col]
}

fn get(b: &Board, c: Coord) -> Option<(Profession, Side)> {
    let (col, row) = foo(c);
    b[row][col]
}

fn apply_parachute(dst: Coord, prof: Profession, s: &State) -> Option<State> {
    if get(&s.b, dst).is_some() {
        /* cannot place on an already occupied square */
        return None;
    }

    match s.whose_turn {
        Sente => {
            /* cannot parachute what you don't have */
            if !s
                .sente_hand
                .iter()
                .any(|cap| prof.to_captured() == Some(*cap))
            {
                return None;
            }

            let mut b = s.b.clone();
            *get_mut(&mut b, dst) = Some((prof, Sente));

            let mut sente_hand = Vec::new();

            let mut already_removed = false;
            for cap in &s.sente_hand {
                if prof.to_captured() == Some(*cap) && !already_removed {
                    already_removed = true;
                    continue;
                }
                sente_hand.push(*cap);
            }

            return Some(State {
                whose_turn: Gote,
                b,
                gote_hand: s.gote_hand.clone(),
                sente_hand,
            });
        }
        Gote => {
            /* cannot parachute what you don't have */
            if !s
                .gote_hand
                .iter()
                .any(|cap| prof.to_captured() == Some(*cap))
            {
                return None;
            }

            let mut b = s.b.clone();
            *get_mut(&mut b, dst) = Some((prof, Gote));

            let mut gote_hand = Vec::new();

            let mut already_removed = false;
            for cap in &s.gote_hand {
                if prof.to_captured() == Some(*cap) && !already_removed {
                    already_removed = true;
                    continue;
                }
                gote_hand.push(*cap);
            }

            return Some(State {
                whose_turn: Gote,
                b,
                sente_hand: s.sente_hand.clone(),
                gote_hand,
            });
        }
    }
}

// Returns the blocking square. That is, even if Some() is returned, you must check that no piece intervenes.
fn validate_move_legality(
    side: Side,
    src: Coord,
    dst: Coord,
    prof: Profession,
) -> Option<Vec<Coord>> {
    let (x1, y1) = foo(src);
    let (x2, y2) = foo(dst);

    let x_diff_abs = (x2 as isize - x1 as isize).abs(); // all movements are symmetric in Shogi

    let y_diff = if side == Sente {
        y2 as isize - y1 as isize
    } else {
        y1 as isize - y2 as isize
    };

    let candidates = match prof {
        Kyo => vec![(0, -1), (0, -2), (0, -3), (0, -4)],
        Gin => vec![(0, -1), (1, 1), (1, -1)],
        Kaku => vec![
            (1, -1),
            (2, -2),
            (3, -3),
            (4, -4),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 4),
        ],
        To | Kin => vec![(0, -1), (0, 1), (1, 0), (1, -1)],
        Kei => vec![(1, -2)],
        Hi => vec![
            (0, -1),
            (0, -2),
            (0, -3),
            (0, -4),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
        ],
        Fu => vec![(0, -1)],
        Ou => vec![(0, -1), (0, 1), (1, 0), (1, 1), (1, -1)],
    };

    if !candidates.contains(&(x_diff_abs, y_diff)) {
        return None;
    }

    let mut ans = Vec::new();
    for coord in &[
        (_1, Yi),
        (_1, Er),
        (_1, San),
        (_1, Si),
        (_1, Wu),
        (_2, Yi),
        (_2, Er),
        (_2, San),
        (_2, Si),
        (_2, Wu),
        (_3, Yi),
        (_3, Er),
        (_3, San),
        (_3, Si),
        (_3, Wu),
        (_4, Yi),
        (_4, Er),
        (_4, San),
        (_4, Si),
        (_4, Wu),
        (_5, Yi),
        (_5, Er),
        (_5, San),
        (_5, Si),
        (_5, Wu),
    ] {
        // let (x1, y1) = foo(src);
        // let (x2, y2) = foo(dst);
        let (x3, y3) = foo(*coord);

        let (v_x, v_y) = (x2 as isize - x1 as isize, y2 as isize - y1 as isize);
        let (w_x, w_y) = (x3 as isize - x1 as isize, y3 as isize - y1 as isize);

        if v_x * w_x + v_y * w_y > 0
            && v_x * w_y - v_y * w_x == 0
            && (v_x * v_x + v_y * v_y) > (w_x * w_x + w_y * w_y)
        {
            ans.push(*coord);
        }
    }

    Some(ans)
}

fn conj(a: Option<(Profession, Side)>) -> Option<(Profession, Side)> {
    match a {
        None => None,
        Some((p, s)) => Some((p.conjugate(), s)),
    }
}

/// ```
/// use kyoto_ginkaku::apply_movement;
/// use kyoto_ginkaku::print_state;
/// use kyoto_ginkaku::M;
/// use kyoto_ginkaku::Side::*;
/// use kyoto_ginkaku::INITIAL;
/// use kyoto_ginkaku::State;
/// use kyoto_ginkaku::StateOrVictory;
/// let mut state = State {
///     b: INITIAL,
///     whose_turn: Sente,
///     sente_hand: vec![],
///     gote_hand: vec![],
/// };
/// 
/// for mov in vec![
///     "☗4四玉(35)",
///     "☖4二玉(31)",
///     "☗5四と(55)",
///     "☖5二歩(51)",
///     "☗5二香(54)",
///     "☖5二玉(42)",
///     "☗5三飛打",
///     "☖4二玉(52)",
///     "☗3四金(25)",
///     "☖3二玉(42)",
///     "☗5四銀(45)",
///     "☖3一玉(32)",
///     "☗3三玉(44)",
///     "☖4二金(41)",
///     "☗4二桂(34)",
/// ] {
///     print_state(&state);
///     println!("\n\nplaying {}:", mov);
///     let m = M(mov);
///     match apply_movement(m, &state).unwrap() {
///         StateOrVictory::State(new_state) => state = new_state,
///         StateOrVictory::Victory(side) => {
///             println!("{:?} victory", side);
///             return;
///         }
///     }
/// }
/// print_state(&state);
/// ```
pub fn apply_movement(m: Movement, s: &State) -> Option<StateOrVictory> {
    if s.whose_turn != m.side {
        return None;
    }

    if let Some(src) = m.src {
        let intervening_squares = validate_move_legality(m.side, src, m.dst, m.prof)?;

        for sq in intervening_squares {
            /* blocked */
            if get(&s.b, sq).is_some() {
                return None;
            }
        }

        match get(&s.b, src) {
            None => return None, /* src cannot be empty */
            Some((prof, side)) => {
                if side != s.whose_turn {
                    return None;
                } /* can't move an opponent's */
                if prof != m.prof {
                    return None;
                } /* the description does not match */
            }
        }

        match get(&s.b, m.dst) {
            None => {
                let mut b = s.b.clone();
                *get_mut(&mut b, m.dst) = conj(get(&s.b, src));
                *get_mut(&mut b, src) = None;
                Some(StateOrVictory::State(State {
                    whose_turn: !s.whose_turn,
                    b,
                    sente_hand: s.sente_hand.clone(),
                    gote_hand: s.gote_hand.clone(),
                }))
            }
            Some((dst_prof, side)) => {
                if side == s.whose_turn {
                    return None;
                } /* can't take your own piece */

                let cap = match dst_prof.to_captured() {
                    None /* 王 */ => return Some(StateOrVictory::Victory(s.whose_turn)),
                    Some(cap) => cap
                };

                let mut b = s.b.clone();
                *get_mut(&mut b, m.dst) = conj(get(&s.b, src));
                *get_mut(&mut b, src) = None;
                Some(StateOrVictory::State(State {
                    whose_turn: !s.whose_turn,
                    b,
                    sente_hand: {
                        let mut v = s.sente_hand.clone();
                        if s.whose_turn == Sente {
                            v.push(cap)
                        }
                        v
                    },
                    gote_hand: {
                        let mut v = s.gote_hand.clone();
                        if s.whose_turn == Gote {
                            v.push(cap)
                        }
                        v
                    },
                }))
            }
        }
    } else {
        Some(StateOrVictory::State(apply_parachute(m.dst, m.prof, &s)?))
    }
}
