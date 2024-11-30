use std::collections::HashSet;

struct Card {
    id: u32,
    winning: HashSet<u32>,
    ours: HashSet<u32>,
}

impl Card {
    fn parse(input: &str) -> Self {
        let id_split = input.split(':').collect::<Vec<_>>();
        let id_part = id_split[0];
        let number_part = id_split[1];

        let id = id_part.split_whitespace().collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();

        let number_split = number_part.split('|').collect::<Vec<_>>();
        let winning = number_split[0]
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let ours = number_split[1]
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        Self { id, winning, ours }
    }

    fn points(&self) -> u32 {
        let intersection_count = self.winning.intersection(&self.ours).count();

        if intersection_count == 0 {
            0
        } else {
            2_u32.pow((intersection_count - 1) as u32)
        }
    }

    fn winning_number_count(&self) -> usize {
        self.winning.intersection(&self.ours).count()
    }
}

fn part_1(input: &str) -> u32 {
    input.lines().map(Card::parse).map(|c| c.points()).sum()
}

fn part_2(input: &str) -> u32 {
    let cards = input.lines().map(Card::parse).collect::<Vec<_>>();
    let mut copy_count = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let copies = copy_count[i];
        let winning_numbers = card.winning_number_count();

        for j in 1..=winning_numbers {
            copy_count[j + i] += copies;
        }
    }

    copy_count.iter().sum()
}

fn main() {
    let sample_input = include_str!("file.txt");
    let sample_part_1_ans = part_1(sample_input);

    dbg!(sample_part_1_ans);

    let my_input = include_str!("file.txt");
    let my_part_1_ans = part_1(my_input);

    dbg!(my_part_1_ans);

    let sample_part_2_ans = part_2(sample_input);
    dbg!(sample_part_2_ans);

    let my_part_2_ans = part_2(my_input);
    dbg!(my_part_2_ans);
}
