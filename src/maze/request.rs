use crate::maze::cell::Coordinates;
use crate::maze::cell::MazeType;
use crate::maze::algorithms::MazeAlgorithm;
use serde::{ Serialize, Deserialize };

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum Algorithm {
//     BinaryTree,
//     Sidewinder,
//     AldousBroder,
//     Wilsons,
//     HuntAndKill,
//     RecursiveBacktracker
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MazeRequest {
    pub maze_type: MazeType,
    pub width: usize,
    pub height: usize,
    pub algorithm: MazeAlgorithm,
    pub start: Coordinates,
    pub goal: Coordinates,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let request = MazeRequest {
            maze_type: MazeType::Orthogonal,
            width: 10,
            height: 10,
            algorithm: MazeAlgorithm::BinaryTree,
            start: Coordinates { x: 0, y: 0 },
            goal: Coordinates { x: 9, y: 9 },
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize MazeRequest");
        assert!(json.contains("\"maze_type\":\"Orthogonal\""));
        assert!(json.contains("\"width\":10"));
        assert!(json.contains("\"height\":10"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 10,
            "height": 10,
            "algorithm": "RecursiveBacktracker",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 9, "y": 9 }
        }
        "#;

        let request: MazeRequest = serde_json::from_str(json).expect("Failed to deserialize MazeRequest");

        assert_eq!(request.maze_type, MazeType::Orthogonal);
        assert_eq!(request.width, 10);
        assert_eq!(request.height, 10);
        assert_eq!(request.algorithm, MazeAlgorithm::RecursiveBacktracker);
        assert_eq!(request.start, Coordinates { x: 0, y: 0 });
        assert_eq!(request.goal, Coordinates { x: 9, y: 9 });
    }
}