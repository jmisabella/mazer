use mazer::{self, generate};

#[test]
fn test_recursive_backtracker_orthogonal_12_x_12_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Orthogonal",
        "width": 12,
        "height": 12,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 11, "y": 11 }
    }
    "#;
    let maze = generate(json);
    assert!(maze.is_perfect_maze());
    println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());

}
