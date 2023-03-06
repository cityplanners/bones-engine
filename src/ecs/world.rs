use std::cell::{RefCell, RefMut, Ref};
use crate::engine::State;
use crate::engine::model::Model;

pub struct World {
    pub state: State,
}

impl World {
    pub fn new(state: State) -> World {
        Self {
            state
        }
    }

    pub fn spawn_entity(&mut self) -> usize {
        let entity_id = self.state.entity_count;
        for component_vec in self.state.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.state.entity_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        // Search for any existing ComponentVecs that match the type of the component being added.
        for component_vec in self.state.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.borrow_mut()[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.state.entity_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.state.entity_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity] = Some(component);
        self.state.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.state.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn load_model(&self, filename: &str) -> Option<Model> {
        let result = pollster::block_on(crate::engine::resources::load_model(filename, &self.state.device, &self.state.queue, &self.state.texture_bind_group_layout));
        match result {
            Ok(model) => {
                Some(model)
            }
            Err(error) => {
                panic!("Failed to load file: {}", error);
            }
        }
    }
}
