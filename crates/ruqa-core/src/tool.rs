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
    /// On-canvas text tool.
    Text,
}

impl Tool {
    /// Array of all 9 primary drawing and manipulation tools in canonical toolbar order.
    pub const ALL: [Tool; 9] = [
        Tool::Select,
        Tool::Pen,
        Tool::Rectangle,
        Tool::Ellipse,
        Tool::Diamond,
        Tool::Line,
        Tool::Arrow,
        Tool::Eraser,
        Tool::Text,
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
            Tool::Text => "Text",
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
            Tool::Text => 'T',
        }
    }

    /// Maps a single-key keyboard shortcut character to its corresponding [`Tool`], if recognized.
    ///
    /// Supports both primary and secondary aliases (e.g. both `'O'` and `'C'` for [`Tool::Ellipse`]).
    #[must_use]
    pub fn from_shortcut(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'V' => Some(Tool::Select),
            'P' => Some(Tool::Pen),
            'R' => Some(Tool::Rectangle),
            'O' | 'C' => Some(Tool::Ellipse),
            'D' => Some(Tool::Diamond),
            'L' => Some(Tool::Line),
            'A' => Some(Tool::Arrow),
            'E' => Some(Tool::Eraser),
            'T' => Some(Tool::Text),
            _ => None,
        }
    }

    /// Returns `true` if this tool interactively creates vector shapes or pen strokes via drag.
    #[must_use]
    pub fn is_drawable(self) -> bool {
        matches!(
            self,
            Tool::Pen | Tool::Rectangle | Tool::Ellipse | Tool::Diamond | Tool::Line | Tool::Arrow
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tools_count_and_uniqueness() {
        assert_eq!(Tool::ALL.len(), 9);
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
    fn test_from_shortcut_mapping() {
        assert_eq!(Tool::from_shortcut('v'), Some(Tool::Select));
        assert_eq!(Tool::from_shortcut('V'), Some(Tool::Select));
        assert_eq!(Tool::from_shortcut('p'), Some(Tool::Pen));
        assert_eq!(Tool::from_shortcut('r'), Some(Tool::Rectangle));
        assert_eq!(Tool::from_shortcut('o'), Some(Tool::Ellipse));
        assert_eq!(Tool::from_shortcut('c'), Some(Tool::Ellipse));
        assert_eq!(Tool::from_shortcut('C'), Some(Tool::Ellipse));
        assert_eq!(Tool::from_shortcut('d'), Some(Tool::Diamond));
        assert_eq!(Tool::from_shortcut('l'), Some(Tool::Line));
        assert_eq!(Tool::from_shortcut('a'), Some(Tool::Arrow));
        assert_eq!(Tool::from_shortcut('e'), Some(Tool::Eraser));
        assert_eq!(Tool::from_shortcut('t'), Some(Tool::Text));
        assert_eq!(Tool::from_shortcut('x'), None);
        assert_eq!(Tool::from_shortcut('1'), None);
    }

    #[test]
    fn test_default_tool_is_select() {
        assert_eq!(Tool::default(), Tool::Select);
    }
}
