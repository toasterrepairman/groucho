// use std::sync::Arc;

// use pyke_diffusers::{
//     EulerDiscreteScheduler, OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionOptions, StableDiffusionPipeline,
//     StableDiffusionTxt2ImgOptions
// };

// pub fn infer(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let environment = Arc::new(OrtEnvironment::builder().build()?);
//     let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default()?;
//     let pipeline = StableDiffusionPipeline::new(&environment, "./stable-diffusion-v1-5", StableDiffusionOptions::default())?;
//     let mut imgs = StableDiffusionTxt2ImgOptions::default()
//         .with_prompt("photo of a red fox")
//         .with_steps(20)
//         .run(&pipeline, &mut scheduler)?;

//     imgs[0].clone().into_rgb8().save("/tmp/result.png")?;
//     Ok(())
// }
