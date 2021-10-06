use std::collections::{HashMap, HashSet};

pub struct Sorter {
    dependencies: HashMap<String, HashSet<String>>,
}

impl Sorter {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }
    pub fn insert(&mut self, dep: String) {
        self.dependencies.insert(dep, HashSet::new());
    }
    pub fn add_dependency(&mut self, dep: String, name: String) {
        if let Some(deps) = self.dependencies.get_mut(&name) {
            deps.insert(dep);
        } else {
            self.dependencies.insert(dep, HashSet::new());
        }
    }

    pub fn remove_unregistered_depedencies(&mut self) {
        let mut deps = HashSet::new();

        for (_, v) in &self.dependencies {
            deps.extend(v);
        }
        deps.retain(|dep| !self.dependencies.contains_key(*dep));
        let deps: HashSet<_> = deps.into_iter().cloned().collect();
        for dep in deps {
            self.remove_dep(&dep.clone());
        }
    }

    fn remove_dep(&mut self, dep: &str) {
        for (k, v) in &mut self.dependencies {
            dbg!(&dep, k, &v);

            v.remove(dep);
        }
    }
    pub fn pop(&mut self) -> Option<String> {
        let indep = self.find_independent()?;
        self.remove_dep(&indep);
        Some(indep)
    }

    fn find_independent(&mut self) -> Option<String> {
        let mut out = None;
        for (k, v) in &self.dependencies {
            if v.is_empty() {
                let key = &k.clone();
                out = self.dependencies.remove_entry(key).map(|x|x.0);
                break;
            }
        }
        let out = out?;
        self.dependencies.remove(&out);
        Some(out)
    }
}
