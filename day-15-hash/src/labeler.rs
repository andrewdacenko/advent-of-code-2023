use crate::hash;

const DASH: &u8 = &b'-';

#[derive(Debug)]
struct Library {
    boxes: Vec<Box>,
}

impl Library {
    fn apply(&mut self, step: &str) {
        match step.as_bytes().last() {
            Some(DASH) => self.remove_operation(step),
            Some(_) => self.assign_operation(step),
            None => panic!("Unexpected operation in step {:?}", step),
        }
    }

    fn box_item(&mut self, label_str: &str) -> &mut Box {
        let index = hash::hash_string(label_str);
        self.boxes
            .get_mut(index)
            .expect(format!("Box should exist {index}").as_str())
    }

    fn remove_operation(&mut self, step: &str) {
        let [label @ .., _operation] = step.as_bytes() else { todo!() };
        let label_str = std::str::from_utf8(label).expect("Must be string");
        self.box_item(label_str).remove_lens(label_str);
    }

    fn assign_operation(&mut self, step: &str) {
        let [label @ .., _operation, focal_length] = step.as_bytes() else { todo!() };
        let label_str = std::str::from_utf8(label).expect("Must be string");
        let focal_length_num: u8 = std::str::from_utf8(&[*focal_length])
            .expect("Must be number")
            .parse::<u8>()
            .unwrap();
        self.box_item(label_str)
            .assign_lens(label_str, focal_length_num);
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_position, box_item)| {
                box_item
                    .lenses
                    .iter()
                    .enumerate()
                    .map(move |(lens_position, lens)| {
                        (box_position + 1) * (lens_position + 1) * lens.focal_length as usize
                    })
            })
            .flatten()
            .sum()
    }
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Clone, Debug)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|lens| lens.label != label.to_string())
    }

    fn assign_lens(&mut self, label: &str, focal_length: u8) {
        if let Some(position) = self
            .lenses
            .iter()
            .position(|lens| lens.label.eq(&label.to_string()))
        {
            self.lenses.get_mut(position).as_mut().unwrap().focal_length = focal_length
        } else {
            self.lenses.push(Lens {
                label: label.to_string(),
                focal_length,
            })
        }
    }
}

pub fn focusing_power(instructions: &str) -> usize {
    let mut library = Library {
        boxes: (0..256).map(|_| Box { lenses: vec![] }).collect(),
    };

    for step in instructions.split(',') {
        library.apply(step)
    }

    return library.focusing_power();
}
