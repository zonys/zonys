use ztd::Constructor;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(super) trait ZoneConfigurationTraversable<T> {
    fn children(&self) -> Vec<T>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(super))]
pub struct ZoneConfigurationTraverser<T> {
    configuration: Vec<T>,
}

impl<T> ZoneConfigurationTraverser<T> {
    pub fn inorder(self) -> ZoneConfigurationInorderTraverser<T> {
        ZoneConfigurationInorderTraverser::new(self.configuration)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Constructor, Debug)]
#[Constructor(visibility = pub(self))]
pub struct ZoneConfigurationInorderTraverser<T> {
    todo: Vec<T>,
}

impl<T> Iterator for ZoneConfigurationInorderTraverser<T>
where
    T: ZoneConfigurationTraversable<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let top = match self.todo.pop() {
            None => return None,
            Some(top) => top,
        };

        self.todo.extend(top.children());

        Some(top)
    }
}
