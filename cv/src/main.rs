use anyhow::Result;

use opencv::{
    self as cv,
    highgui
};

fn main() -> Result <()>{
    println!("Hello, world!");
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    
    let a : [i32; 400] = (1..=400).collect::<Vec<_>>()
        .try_into().expect("wrong size iterator");
    
    let mut i = 0;
    
    loop {
        let elem = a[i];
        let xdim = (elem as f64).sqrt() as i32;
        let ydim = if xdim * xdim == elem {xdim} else { ( (elem as f64) / (xdim as f64)).ceil() as i32 };
        let mut im = unsafe { cv::core::Mat::new_rows_cols_with_default(2000, 2000,
                                                                        cv::core::CV_8UC4, cv::core::VecN([0., 0., 0., 0.]))? };

        
        for j in 0..elem {
            let x = (j % xdim) * 100;
            let y = (j / xdim) * 100;

            cv::imgproc::rectangle(
                &mut im,
                cv::core::Rect::from_points(cv::core::Point::new(x+5, y+5), cv::core::Point::new(x+95, y+95)),
                cv::core::VecN([255., 0., 0., 0.]),
                -1,
                cv::imgproc::LINE_8,
                0
            )?;
        }
        
        highgui::imshow("window", &im)?;
        let key = highgui::wait_key(20)?;
        
        if key == 113 {
            break;
        }

        i = (i + 1) % 400;

    }
    
    Ok(())
}
