// https://users.rust-lang.org/t/what-is-the-best-way-to-mutate-a-collection-whilst-iterating-over-it-with-nested-loops/101808
// https://ryhl.io/blog/temporary-shared-mutation
// https://users.rust-lang.org/t/nested-iteration-within-mutable-iteration/14594
// https://www.reddit.com/r/learnrust/comments/s3qda2/is_it_possible_to_have_nested_iterations_over_a
// https://users.rust-lang.org/t/how-to-mutably-loop-over-vector-in-a-nested-loop/28601
// https://stackoverflow.com/questions/25960318/idiomatic-iterator-mutation-on-a-struct-in-rust

fn main() {}

#[derive(Debug, Clone)]
struct Entity {
    path: String,
    nested: Vec<String>,
}

impl Entity {
    fn new(path: impl Into<String>) -> Self {
        Self { path: path.into(), nested: vec![] }
    }
}

// fn detect_nested_paths(entities: &mut Vec<Entity>) {
//     for entity1 in entities.iter() {
//         for entity2 in entities.iter_mut() {
//             if entity1.path.starts_with(&entity2.path) && entity1.path != entity2.path {
//                 entity2.nested.push(entity1.path.clone());
//             }
//         }
//     }
// }
//
// error[E0502]: cannot borrow `*entities` as mutable because it is also borrowed as immutable
//   --> my/experiments/src/bin/mutation_in_nested_loop.rs:17:24
//    |
// 16 |     for entity1 in entities.iter() {
//    |                    ---------------
//    |                    |
//    |                    immutable borrow occurs here
//    |                    immutable borrow later used here
// 17 |         for entity2 in entities.iter_mut() {
//    |                        ^^^^^^^^ mutable borrow occurs here

fn detect_nested_paths_1(entities: &mut Vec<Entity>) {
    for i in 0..entities.len() {
        for j in 0..entities.len() {
            if entities[i].path.starts_with(&entities[j].path)
                && entities[i].path != entities[j].path
            {
                let path = entities[i].path.clone();
                entities[j].nested.push(path);
            }
        }
    }
}

fn detect_nested_paths_2(entities: &mut Vec<Entity>) {
    for i in 0..entities.len() {
        let mut nested = vec![];
        for entity in entities.iter() {
            if entity.path.starts_with(&entities[i].path) && entity.path != entities[i].path {
                let path = entity.path.clone();
                nested.push(path);
            }
        }
        entities[i].nested = nested;
    }
}

fn detect_nested_paths_3(entities: &mut Vec<Entity>) {
    let paths: Vec<_> = entities.iter().map(|entity| entity.path.clone()).collect();
    for path in paths.iter() {
        for entity in entities.iter_mut() {
            if path.starts_with(&entity.path) && *path != entity.path {
                entity.nested.push(path.clone());
            }
        }
    }
}

fn detect_nested_paths_4(entities: &mut Vec<Entity>) {
    let entities: Vec<_> = entities.into_iter().map(std::cell::RefCell::from).collect();
    for entity1 in entities.iter() {
        for entity2 in entities.iter() {
            if entity1.borrow().path.starts_with(&entity2.borrow().path)
                && entity1.borrow().path != entity2.borrow().path
            {
                entity2.borrow_mut().nested.push(entity1.borrow().path.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_detect_nested_paths() {
        let entities = vec![
            Entity::new("/"),
            Entity::new("/vol1"),
            Entity::new("/vol1/vol2"),
            Entity::new("/vol1/vol2/vol3"),
            Entity::new("/vol1/vol4"),
            Entity::new("/vol5"),
        ];
        let solutions = vec![
            detect_nested_paths_1,
            detect_nested_paths_2,
            detect_nested_paths_3,
            detect_nested_paths_4,
        ];
        for solution in solutions {
            let mut entities = entities.clone();
            solution(&mut entities);
            let expected = expect![[r#"
            [
                Entity {
                    path: "/",
                    nested: [
                        "/vol1",
                        "/vol1/vol2",
                        "/vol1/vol2/vol3",
                        "/vol1/vol4",
                        "/vol5",
                    ],
                },
                Entity {
                    path: "/vol1",
                    nested: [
                        "/vol1/vol2",
                        "/vol1/vol2/vol3",
                        "/vol1/vol4",
                    ],
                },
                Entity {
                    path: "/vol1/vol2",
                    nested: [
                        "/vol1/vol2/vol3",
                    ],
                },
                Entity {
                    path: "/vol1/vol2/vol3",
                    nested: [],
                },
                Entity {
                    path: "/vol1/vol4",
                    nested: [],
                },
                Entity {
                    path: "/vol5",
                    nested: [],
                },
            ]
        "#]];
            expected.assert_debug_eq(&entities);
        }
    }
}
