use crate::cell::Coordinates;
use crate::cell::MazeType;
use crate::algorithms::MazeAlgorithm;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MazeRequest {
    pub maze_type: MazeType,
    pub width: usize,
    pub height: usize,
    pub algorithm: MazeAlgorithm,
    pub start: Option<Coordinates>,
    pub goal: Option<Coordinates>,
    pub capture_steps: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_of_binary_tree_orthogonal() {
        let request = MazeRequest {
            maze_type: MazeType::Orthogonal,
            width: 10,
            height: 10,
            algorithm: MazeAlgorithm::BinaryTree,
            start: Some(Coordinates { x: 0, y: 0 }),
            goal: Some(Coordinates { x: 9, y: 9 }),
            capture_steps: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize MazeRequest");
        assert!(json.contains("\"maze_type\":\"Orthogonal\""));
        assert!(json.contains("\"width\":10"));
        assert!(json.contains("\"height\":10"));
    }

    #[test]
    fn test_deserialization_of_recursive_backtracker_orthogonal() {
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
        assert_eq!(request.start, Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(request.goal, Some(Coordinates { x: 9, y: 9 }));
    }

    #[test]
    fn test_serialization_of_ellers_orthogonal() {
        let request = MazeRequest {
            maze_type: MazeType::Orthogonal,
            width: 10,
            height: 10,
            algorithm: MazeAlgorithm::Ellers,
            start: Some(Coordinates { x: 0, y: 0 }),
            goal: Some(Coordinates { x: 9, y: 9 }),
            capture_steps: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize MazeRequest");
        assert!(json.contains("\"algorithm\":\"Ellers\""));
        assert!(json.contains("\"width\":10"));
        assert!(json.contains("\"height\":10"));
    }

    #[test]
    fn test_deserialization_of_recursive_division_sigma() {
        let json = r#"
        {
            "maze_type": "Sigma",
            "width": 10,
            "height": 10,
            "algorithm": "RecursiveDivision",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 9, "y": 9 }
        }
        "#;

        let request: MazeRequest = serde_json::from_str(json).expect("Failed to deserialize MazeRequest");

        assert_eq!(request.maze_type, MazeType::Sigma);
        assert_eq!(request.width, 10);
        assert_eq!(request.height, 10);
        assert_eq!(request.algorithm, MazeAlgorithm::RecursiveDivision);
        assert_eq!(request.start, Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(request.goal, Some(Coordinates { x: 9, y: 9 }));
    }

    #[test]
    fn test_serialization_of_growing_tree_random() {
        let request = MazeRequest {
            maze_type: MazeType::Orthogonal,
            width: 10,
            height: 10,
            algorithm: MazeAlgorithm::GrowingTreeRandom,
            start: Some(Coordinates { x: 0, y: 0 }),
            goal: Some(Coordinates { x: 9, y: 9 }),
            capture_steps: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize MazeRequest");
        assert!(json.contains("\"algorithm\":\"GrowingTreeRandom\""));
    }

    #[test]
    fn test_serialization_of_growing_tree_newest() {
        let request = MazeRequest {
            maze_type: MazeType::Orthogonal,
            width: 10,
            height: 10,
            algorithm: MazeAlgorithm::GrowingTreeNewest,
            start: Some(Coordinates { x: 0, y: 0 }),
            goal: Some(Coordinates { x: 9, y: 9 }),
            capture_steps: None,
        };

        let json = serde_json::to_string(&request).expect("Failed to serialize MazeRequest");
        assert!(json.contains("\"algorithm\":\"GrowingTreeNewest\""));
    }

    #[test]
    fn test_deserialization_of_growing_tree_random() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 10,
            "height": 10,
            "algorithm": "GrowingTreeRandom",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 9, "y": 9 }
        }
        "#;

        let request: MazeRequest = serde_json::from_str(json).expect("Failed to deserialize MazeRequest");

        assert_eq!(request.maze_type, MazeType::Orthogonal);
        assert_eq!(request.width, 10);
        assert_eq!(request.height, 10);
        assert_eq!(request.algorithm, MazeAlgorithm::GrowingTreeRandom);
        assert_eq!(request.start, Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(request.goal, Some(Coordinates { x: 9, y: 9 }));
    }

    #[test]
    fn test_deserialization_of_growing_tree_newest() {
        let json = r#"
        {
            "maze_type": "Orthogonal",
            "width": 10,
            "height": 10,
            "algorithm": "GrowingTreeNewest",
            "start": { "x": 0, "y": 0 },
            "goal": { "x": 9, "y": 9 }
        }
        "#;

        let request: MazeRequest = serde_json::from_str(json).expect("Failed to deserialize MazeRequest");

        assert_eq!(request.maze_type, MazeType::Orthogonal);
        assert_eq!(request.width, 10);
        assert_eq!(request.height, 10);
        assert_eq!(request.algorithm, MazeAlgorithm::GrowingTreeNewest);
        assert_eq!(request.start, Some(Coordinates { x: 0, y: 0 }));
        assert_eq!(request.goal, Some(Coordinates { x: 9, y: 9 }));
    }

}