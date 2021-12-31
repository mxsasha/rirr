#![allow(clippy::wrong_self_convention)]
use diesel::expression::{AsExpression, Expression};
use diesel::pg::Pg;

diesel_infix_operator!(IsContainedBy, " << ", backend: Pg);
diesel_infix_operator!(IsContainedByOrEquals, " <<= ", backend: Pg);
diesel_infix_operator!(Contains, " >> ", backend: Pg);
diesel_infix_operator!(ContainsOrEquals, " >>= ", backend: Pg);
diesel_infix_operator!(ContainsOrIsContainedBy, " && ", backend: Pg);

/// From https://github.com/JakubOnderka/ip_network/blob/master/src/diesel_support.rs
pub trait PqCidrExtensionMethods: Expression + Sized {
    /// Creates a SQL `<<` expression.
    fn is_contained_by<T>(self, other: T) -> IsContainedBy<Self, T::Expression>
    where
        T: AsExpression<Self::SqlType>,
    {
        IsContainedBy::new(self, other.as_expression())
    }

    // /// Creates a SQL `<<=` expression.
    fn is_contained_by_or_equals<T>(self, other: T) -> IsContainedByOrEquals<Self, T::Expression>
    where
        T: AsExpression<Self::SqlType>,
    {
        IsContainedByOrEquals::new(self, other.as_expression())
    }

    // /// Creates a SQL `>>` expression.
    fn contains<T>(self, other: T) -> Contains<Self, T::Expression>
    where
        T: AsExpression<Self::SqlType>,
    {
        Contains::new(self, other.as_expression())
    }

    // /// Creates a SQL `>>=` expression.
    fn contains_or_equals<T>(self, other: T) -> ContainsOrEquals<Self, T::Expression>
    where
        T: AsExpression<Self::SqlType>,
    {
        ContainsOrEquals::new(self, other.as_expression())
    }

    // /// Creates a SQL `&&` expression.
    fn contains_or_is_contained_by<T>(
        self,
        other: T,
    ) -> ContainsOrIsContainedBy<Self, T::Expression>
    where
        T: AsExpression<Self::SqlType>,
    {
        ContainsOrIsContainedBy::new(self, other.as_expression())
    }
}
impl<T: Expression> PqCidrExtensionMethods for T {}
