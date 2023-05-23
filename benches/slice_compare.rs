#![feature(test)]

use fastcpy::slice_copy;
use paste::paste;
use test::Bencher;

extern crate test;

fn random_data(size: usize) -> Vec<u8> {
    (0..size).map(|_| 4u8).collect()
}

macro_rules! bench_for_size {
    ($length:expr) => {
        paste! {
            #[bench]
            fn [<bench_ $length _default_copy>](b: &mut Bencher) {
                let data = vec![$length];
                let size = data[0];
                let left = random_data(size);
                let mut right = vec![0u8; size];

                b.iter(|| {
                    for _ in 0..13 {
                        test::black_box(right.copy_from_slice(&left));
                    }
                });
            }

            #[bench]
            fn [<bench_ $length _fast_copy>] (b: &mut Bencher) {
                let data = vec![$length];
                let size = data[0];
                let left = random_data(size);
                let mut right = vec![0u8; size];

                b.iter(|| {
                    for _ in 0..13 {
                        test::black_box(slice_copy(&left, &mut right));
                    }
                });
            }
        }
    };
}

bench_for_size!(1);
bench_for_size!(2);
bench_for_size!(3);
bench_for_size!(4);
bench_for_size!(5);
bench_for_size!(6);
bench_for_size!(7);
bench_for_size!(8);
bench_for_size!(9);
bench_for_size!(10);
bench_for_size!(11);
bench_for_size!(12);
bench_for_size!(13);
bench_for_size!(14);
bench_for_size!(15);
bench_for_size!(16);
bench_for_size!(17);
bench_for_size!(18);
bench_for_size!(22);
bench_for_size!(24);
bench_for_size!(27);
bench_for_size!(30);
bench_for_size!(32);
bench_for_size!(33);
bench_for_size!(64);
bench_for_size!(65);
bench_for_size!(140);
bench_for_size!(1400);
bench_for_size!(14000);
bench_for_size!(140000);
bench_for_size!(1400000);
bench_for_size!(14000000);

#[bench]
fn bench_default_copy(b: &mut Bencher) {
    let sizes = [8, 16, 32, 64, 128, 256, 512, 1024];

    for &size in &sizes {
        let left = random_data(size);
        let mut right = vec![0u8; size];

        b.iter(|| {
            test::black_box(right.copy_from_slice(&left));
        });
    }
}

#[bench]
fn bench_fast_short_slice_copy(b: &mut Bencher) {
    let sizes = [8, 16, 32, 64, 128, 256, 512, 1024];

    for &size in &sizes {
        let left = random_data(size);
        let mut right = vec![0u8; size];

        b.iter(|| {
            test::black_box(slice_copy(&left, &mut right));
        });
    }
}
