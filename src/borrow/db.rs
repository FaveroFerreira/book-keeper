use uuid::Uuid;

use crate::borrow::model::{Borrow, BorrowDTO};
use crate::config::Ctx;

pub async fn update_borrow_partial_by_id() {
    todo!()
}

pub async fn insert_new_borrow(_crx: &Ctx, _borrow: BorrowDTO) -> Borrow {
    todo!()
}

pub async fn find_pending_borrows_by_book(_cxt: &Ctx, _book_id: Uuid) -> Vec<Borrow> {
    todo!()
}

pub async fn find_pending_borrows_by_student(_ctx: &Ctx, _student_id: Uuid) -> Vec<Borrow> {
    todo!()
}
