//! Active drawing and manipulation tool definitions.

use serde::{Deserialize, Serialize};

/// Interactive canvas tools available in Ruqa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Tool {
    /// Selection tool for marquee picking and transforming existing elements.
    #[default]
    Select,
    /// Freehand drawing pen with velocity-based pressure smoothing.
    Pen,
    /// Hand-drawn rough rectangle tool.
    Rectangle,
    /// Hand-drawn rough ellipse tool.
    Ellipse,
    /// Hand-drawn rough diamond tool.
    Diamond,
    /// Hand-drawn straight line tool.
    Line,
    /// Hand-drawn directed arrow tool.
    Arrow,
    /// Continuous element eraser tool.
    Eraser,
}

impl Tool {
    /// Array of all 8 primary drawing and manipulation tools in canonical toolbar order.
    pub const ALL: [Tool; 8] = [
        Tool::Select,
        Tool::Pen,
        Tool::Rectangle,
        Tool::Ellipse,
        Tool::Diamond,
        Tool::Line,
        Tool::Arrow,
        Tool::Eraser,
    ];

    /// Returns the human-readable display label for this tool.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Pen => "Pen",
            Tool::Rectangle => "Rectangle",
            Tool::Ellipse => "Ellipse",
            Tool::Diamond => "Diamond",
            Tool::Line => "Line",
            Tool::Arrow => "Arrow",
            Tool::Eraser => "Eraser",
        }
    }

    /// Returns the standard single-key keyboard shortcut for this tool.
    #[must_use]
    pub fn shortcut(self) -> char {
        match self {
            Tool::Select => 'V',
            Tool::Pen => 'P',
            Tool::Rectangle => 'R',
            Tool::Ellipse => 'O',
            Tool::Diamond => 'D',
            Tool::Line => 'L',
            Tool::Arrow => 'A',
            Tool::Eraser => 'E',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tools_count_and_uniqueness() {
        assert_eq!(Tool::ALL.len(), 8);
        for (i, t1) in Tool::ALL.iter().enumerate() {
            for (j, t2) in Tool::ALL.iter().enumerate() {
                if i != j {
                    assert_ne!(t1, t2);
                    assert_ne!(t1.shortcut(), t2.shortcut());
                }
            }
        }
    }

    #[test]
    fn test_default_tool_is_select() {
        assert_eq!(Tool::default(), Tool::Select);
    }
}
