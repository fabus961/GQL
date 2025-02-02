use std::cmp;

use crate::engine_function::select_gql_objects;
use crate::expression::Expression;
use crate::object::GQLObject;

pub trait Statement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<GQLObject>);
}

pub struct SelectStatement {
    pub table_name: String,
    pub fields: Vec<String>,
}

impl Statement for SelectStatement {
    fn execute(&self, repo: &git2::Repository, objects: &mut Vec<GQLObject>) {
        let elements =
            select_gql_objects(repo, self.table_name.to_string(), self.fields.to_owned());
        for element in elements {
            objects.push(element);
        }
    }
}

pub struct WhereStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for WhereStatement {
    fn execute(&self, _repo: &git2::Repository, objects: &mut Vec<GQLObject>) {
        let result: Vec<GQLObject> = objects
            .iter()
            .filter(|&object| self.condition.evaluate(object).eq("true"))
            .cloned()
            .collect();

        objects.clear();

        for object in result {
            objects.push(object);
        }
    }
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn execute(&self, _repo: &git2::Repository, objects: &mut Vec<GQLObject>) {
        if self.count <= objects.len() {
            objects.drain(self.count..objects.len());
        }
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn execute(&self, _repo: &git2::Repository, objects: &mut Vec<GQLObject>) {
        objects.drain(0..cmp::min(self.count, objects.len()));
    }
}

pub struct OrderByStatement {
    pub field_name: String,
}

impl Statement for OrderByStatement {
    fn execute(&self, _repo: &git2::Repository, objects: &mut Vec<GQLObject>) {
        if objects.is_empty() {
            return;
        }

        if objects[0].attributes.contains_key(&self.field_name) {
            objects.sort_by_key(|object| {
                object
                    .attributes
                    .get(&self.field_name.to_string())
                    .unwrap()
                    .to_string()
            });
        }
    }
}
