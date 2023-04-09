CREATE TABLE borrow(
    id UUID NOT NULL PRIMARY KEY,
    return_date DATE,
    student_id UUID,
    book_id UUID
);
