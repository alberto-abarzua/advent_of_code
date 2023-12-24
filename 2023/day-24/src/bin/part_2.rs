use z3::ast::Ast;

fn main() {
    println!("Solution:\n{:?}", solution(include_str!("../input.txt")));
}

#[derive(Debug, Clone)]
struct Stone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Stone {
    fn new(pos: (i64, i64, i64), vel: (i64, i64, i64)) -> Stone {
        Stone { pos, vel }
    }
}

fn solution(input: &str) -> i64 {
    // This solution is very very inspired by the reddit thread (Basically just used the z3 solver)
    let stones: Vec<Stone> = input
        .lines()
        .map(|l| {
            let l = l.replace("  ", " ");
            let mut values = l.split(" @ ").map(|s| {
                s.split(", ")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            });
            let pos = values.next().unwrap();
            let speed = values.next().unwrap();
            Stone::new((pos[0], pos[1], pos[2]), (speed[0], speed[1], speed[2]))
        })
        .collect();

    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = z3::ast::Int::new_const(&context, "x");
    let y = z3::ast::Int::new_const(&context, "y");
    let z = z3::ast::Int::new_const(&context, "z");
    let vx = z3::ast::Int::new_const(&context, "vx");
    let vy = z3::ast::Int::new_const(&context, "vy");
    let vz = z3::ast::Int::new_const(&context, "vz");

    for (i, hs) in stones.iter().take(3).enumerate() {
        let a = z3::ast::Int::from_i64(&context, hs.pos.0);
        let va = z3::ast::Int::from_i64(&context, hs.vel.0);
        let b = z3::ast::Int::from_i64(&context, hs.pos.1);
        let vb = z3::ast::Int::from_i64(&context, hs.vel.1);
        let c = z3::ast::Int::from_i64(&context, hs.pos.2);
        let vc = z3::ast::Int::from_i64(&context, hs.vel.2);

        let t = z3::ast::Int::new_const(&context, format!("t{i}"));
        solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
        solver.assert(&(x.clone() + vx.clone() * t.clone())._eq(&(a + va * t.clone())));
        solver.assert(&(y.clone() + vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
        solver.assert(&(z.clone() + vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
    }
    if solver.check() == z3::SatResult::Sat {
        let Some(m) = solver.get_model() else {
            println!("Failed to solve!");
            return 0;
        };
        return m.eval(&(x + y + z), true).unwrap().as_i64().unwrap();
    }
    0
}
