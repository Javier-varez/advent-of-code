fn main() -> anyhow::Result<()> {
    let sum = std::io::stdin()
        .lines()
        .fold(Ok(0), |acc: anyhow::Result<isize>, line| {
            let acc = acc?;
            let line = line?;

            let history = line
                .split_whitespace()
                .map(|v| Ok(v.parse()?))
                .collect::<anyhow::Result<Vec<isize>>>()?;

            let mut diffs = vec![history];

            while !diffs
                .last()
                .expect("At least one element")
                .iter()
                .all(|v| *v == 0)
            {
                let diff = diffs
                    .last()
                    .expect("At least one element")
                    .iter()
                    .fold((vec![], None), |(mut v, p), elem| match p {
                        None => (v, Some(elem)),
                        Some(p) => {
                            v.push(elem - p);
                            (v, Some(elem))
                        }
                    })
                    .0;

                diffs.push(diff);
            }

            let mut decr = 0;
            for diff in diffs.iter().rev().skip(1) {
                decr = diff.first().expect("At least one value") - decr;
            }

            Ok(acc + decr)
        })?;

    println!("sum {sum}");
    Ok(())
}
