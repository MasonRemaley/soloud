use soloud_rust::{Builder, BuilderFlags, Queue, SoLoud, Speech, Wav};
use std::f32::consts::PI;

pub fn main() {
    let mut soloud = Builder::new()
        .flags(BuilderFlags::CLIP_ROUNDOFF | BuilderFlags::ENABLE_VISUALIZATION)
        .init();

    speech_test(&mut soloud);
    queue_test(&mut soloud);
}

fn speech_test(soloud: &mut SoLoud) {
    let mut speech = Speech::new();
    speech.set_text("1 2 3       A B C        Doooooo    Reeeeee    Miiiiii    Faaaaaa    Soooooo    Laaaaaa    Tiiiiii    Doooooo!").unwrap();
    soloud.play(&speech);

    println!("Playing speech test..");
    let mut spin = 0;
    while soloud.voice_count() > 0 {
        visualize_volume(soloud, &mut spin);
    }
    println!("\nFinished.");
}

fn queue_test(soloud: &mut SoLoud) {
    let mut queue = Queue::new();
    let mut wav = Vec::with_capacity(4);
    for _ in 0..wav.capacity() {
        wav.push(Wav::new());
    }
    let mut buf = vec![0.0; 2048];

    soloud.play(&queue);

    let mut count = 0;
    for i in 0..4 {
        generate_sample(&mut buf, &mut count);
        wav[i].load_raw_wave(&buf, 44100.0, 1);
        assert_eq!(queue.play(&wav[i]), 0);
    }

    println!("Playing queue / wav generation test..");

    let mut spin = 0;
    let mut cycle = 0;
    while count < 44100 * 10 && soloud.voice_count() > 0 {
        if queue.queue_count() < 3 {
            generate_sample(&mut buf, &mut count);
            wav[cycle].load_raw_wave(&buf, 44100.0, 1);
            assert_eq!(queue.play(&wav[cycle]), 0);
            cycle = (cycle + 1) % 4;
        }
        visualize_volume(soloud, &mut spin);
    }

    while soloud.voice_count() > 0 {
        visualize_volume(soloud, &mut spin);
    }

    println!("\nFinished.");
}

fn visualize_volume(soloud: &mut SoLoud, spin: &mut i32) {
    let v = soloud.approximate_volume(0);
    print!("\r{} ", ['|', '\\', '-', '/'][(*spin & 3) as usize]);
    *spin += 1;
    let mut p = (v * 60.0) as i32;
    if p > 59 {
        p = 59;
    }
    for _ in 0..p {
        print!("=");
    }
    for _ in p..60 {
        print!(" ");
    }
}

fn generate_sample(buf: &mut [f32], count: &mut i32) {
    let mut i = 0;
    let mut base = *count;
    while i < 2048 {
        buf[i] = (220.0 * PI * 2.0 * base as f32 * (1.0 / 44100.0)).sin()
            - (230.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        buf[i] += (((unsafe { libc::rand() } % 1024) - 512) as f32 / 512.0)
            * (60.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin()
            * (1.0 * PI * 2.0 * (base as f32) * (1.0 / 44100.0)).sin();
        let fade = (44100.0 * 10.0 - (base as f32)) / (44100.0 * 10.0);
        buf[i] *= fade * fade;

        i += 1;
        base += 1;
    }
    *count = base;
}
