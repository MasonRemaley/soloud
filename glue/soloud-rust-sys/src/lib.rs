// #include <stdlib.h>
// #include <stdio.h>
// #include <math.h>
// #include "soloud_c.h"

// TODO(mr): Specify which library we're linking to here
extern "C" {
    fn Soloud_create();
}

// TODO(mr): Move this out into demo.rs
pub fn run_demo() {
    unsafe {
        let soloud = Soloud_create();

        // Soloud_initEx(soloud, SOLOUD_CLIP_ROUNDOFF | SOLOUD_ENABLE_VISUALIZATION, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO, SOLOUD_AUTO);

        // speech_test(soloud);
        // queue_test(soloud);

        // Soloud_deinit(soloud);
            
        // Soloud_destroy(soloud);

        println!("Cleanup done.");
    }
}

// void visualize_volume(Soloud *soloud)
// {
//     static int spin = 0;
//     int i, p;
//     float v = Soloud_getApproximateVolume(soloud, 0);
//     printf("\r%c ", (int)("|\\-/"[spin & 3]));
//     spin++;
//     p = (int)(v * 60);
//     if (p > 59) p = 59;
//     for (i = 0; i < p; i++)
//         printf("=");
//     for (i = p; i < 60; i++)
//         printf(" ");
// }

// void speech_test(Soloud *soloud)
// {
//     Speech *speech = Speech_create();

//     Speech_setText(speech, "1 2 3       A B C        Doooooo    Reeeeee    Miiiiii    Faaaaaa    Soooooo    Laaaaaa    Tiiiiii    Doooooo!");

//     Soloud_setGlobalVolume(soloud, 4);
//     Soloud_play(soloud, speech);

//     printf("Playing speech test..\n");

//     while (Soloud_getVoiceCount(soloud) > 0)
//     {
//         visualize_volume(soloud);
//     }
//     printf("\nFinished.\n");
//     Speech_destroy(speech);
// }

// void generate_sample(float *buf, int *count)
// {
//     int i;
//     int base = *count;
//     for (i = 0; i < 2048; i++, base++)
//     {
//         buf[i] = (float)sin(220 * 3.14 * 2 * base * (1 / 44100.0)) -
//                  (float)sin(230 * 3.14 * 2 * base * (1 / 44100.0));
//         buf[i] += (((rand() % 1024) - 512) / 512.0f) *
//                   (float)sin(60 * 3.14 * 2 * base * (1 / 44100.0)) *
//                   (float)sin(1 * 3.14 * 2 * base * (1 / 44100.0));
//         float fade = (44100 * 10 - base) / (44100 * 10.0f);
//         buf[i] *= fade * fade;
//     }
//     *count = base;
// }

// void queue_test(Soloud *soloud)
// {
//     int i;
//     int count = 0;
//     int cycle = 0;
//     Queue *queue = Queue_create();  
//     Wav *wav[4];
//     float buf[2048];
//     for (i = 0; i < 4; i++)
//         wav[i] = Wav_create();
//     for (i = 0; i < 2048; i++)
//         buf[i] = 0;

//     Soloud_play(soloud, queue);

//     for (i = 0; i < 4; i++)
//     {
//         generate_sample(buf, &count);
//         Wav_loadRawWaveEx(wav[i], buf, 2048, 44100, 1, 1, 0);
//         Queue_play(queue, wav[i]);
//     }

//     printf("Playing queue / wav generation test..\n");

//     while (count < 44100 * 10 && Soloud_getVoiceCount(soloud) > 0)
//     {
//         if (Queue_getQueueCount(queue) < 3)
//         {
//             generate_sample(buf, &count);
//             Wav_loadRawWaveEx(wav[cycle], buf, 2048, 44100, 1, 1, 0);
//             Queue_play(queue, wav[cycle]);
//             cycle = (cycle + 1) % 4;
//         }
//         visualize_volume(soloud);
//     }

//     while (Soloud_getVoiceCount(soloud) > 0)
//     {
//         visualize_volume(soloud);
//     }

//     printf("\nFinished.\n");

//     for (i = 0; i < 4; i++)
//         Wav_destroy(wav[i]);
//     Queue_destroy(queue);
// }

// TODO(mr): Add some sort of basic tests or just rely on whatever has already been set up?
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
