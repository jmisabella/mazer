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
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
            println!("\n\nRecursive Backtracker\n\n{}\n\n", maze.to_asci());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_aldous_broder_delta_16_x_16_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Delta",
        "width": 16,
        "height": 16,
        "algorithm": "AldousBroder",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 15, "y": 15 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    };
}

#[test]
fn test_hunt_and_kill_sigma_26_x_26_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Sigma",
        "width": 26,
        "height": 26,
        "algorithm": "HuntAndKill",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 25, "y": 25 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    };
}

#[test]
fn test_aldous_broder_orthogonal_12_x_24_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Orthogonal",
        "width": 12,
        "height": 24,
        "algorithm": "AldousBroder",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 11, "y": 23 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
            println!("{}", maze.to_asci());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_aldous_broder_delta_12_x_24_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Delta",
        "width": 12,
        "height": 24,
        "algorithm": "AldousBroder",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 11, "y": 23 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}


#[test]
fn test_aldous_broder_sigma_12_x_24_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Sigma",
        "width": 12,
        "height": 24,
        "algorithm": "AldousBroder",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 11, "y": 23 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_recursive_backtracker_delta_24_x_12_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Delta",
        "width": 24,
        "height": 12,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 23, "y": 11 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_recursive_backtracker_upsilon_24_x_12_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Upsilon",
        "width": 24,
        "height": 12,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 23, "y": 11 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_recursive_backtracker_rhombille_24_x_12_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Rhombille",
        "width": 24,
        "height": 12,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 23, "y": 11 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

#[test]
fn test_aldous_broder_rhombille_23_x_11_maze_generation_from_json() {
    let json = r#"
    {
        "maze_type": "Rhombille",
        "width": 23,
        "height": 11,
        "algorithm": "RecursiveBacktracker",
        "start": { "x": 0, "y": 0 },
        "goal": { "x": 22, "y": 10 }
    }
    "#;
    match generate(json) {
        Ok(maze) => {
            assert!(maze.is_perfect_maze().unwrap());
        }
        Err(e) => {
            dbg!(&e);
            panic!("Error occured: {}", e);
        }
    }
}

