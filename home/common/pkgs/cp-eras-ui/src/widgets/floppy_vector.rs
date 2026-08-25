use iced::widget::canvas;
use iced::Color;

/// Draw the vectorized floppy (selected)
pub fn draw_selected(frame: &mut canvas::Frame) {
    let red = Color::new(0.953, 0.204, 0.204, 1.0);
    let darkest_blue = Color::new(0.055, 0.055, 0.090, 1.0);

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(138.00, 35.00));
        builder.line_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.line_to(iced::Point::new(20.00, 141.00));
        builder.close();
    });
    frame.fill(&path, darkest_blue);

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(102.00, 185.00));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.line_to(iced::Point::new(20.00, 141.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.05, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(102.00, 185.00));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.40, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(211.80, 87.90));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.40, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(138.00, 35.00));
        builder.line_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(102.00, 185.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.70, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(27.00, 132.32));
        builder.line_to(iced::Point::new(136.74, 40.25));
        builder.line_to(iced::Point::new(205.37, 82.94));
        builder.line_to(iced::Point::new(202.03, 96.89));
        builder.line_to(iced::Point::new(103.26, 179.75));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.05, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(62.12, 102.86));
        builder.line_to(iced::Point::new(95.04, 75.24));
        builder.line_to(iced::Point::new(127.83, 95.63));
        builder.line_to(iced::Point::new(94.91, 123.25));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.70, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(38.00, 142.00));
        builder.line_to(iced::Point::new(94.00, 176.00));
        builder.line_to(iced::Point::new(137.00, 140.00));
        builder.line_to(iced::Point::new(81.00, 106.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.70, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(165.00, 92.00));
        builder.line_to(iced::Point::new(164.63, 94.82));
        builder.line_to(iced::Point::new(163.53, 97.56));
        builder.line_to(iced::Point::new(161.73, 100.17));
        builder.line_to(iced::Point::new(159.27, 102.58));
        builder.line_to(iced::Point::new(156.21, 104.73));
        builder.line_to(iced::Point::new(152.63, 106.56));
        builder.line_to(iced::Point::new(148.62, 108.04));
        builder.line_to(iced::Point::new(144.27, 109.12));
        builder.line_to(iced::Point::new(139.69, 109.78));
        builder.line_to(iced::Point::new(135.00, 110.00));
        builder.line_to(iced::Point::new(130.31, 109.78));
        builder.line_to(iced::Point::new(125.73, 109.12));
        builder.line_to(iced::Point::new(121.38, 108.04));
        builder.line_to(iced::Point::new(117.37, 106.56));
        builder.line_to(iced::Point::new(113.79, 104.73));
        builder.line_to(iced::Point::new(110.73, 102.58));
        builder.line_to(iced::Point::new(108.27, 100.17));
        builder.line_to(iced::Point::new(106.47, 97.56));
        builder.line_to(iced::Point::new(105.37, 94.82));
        builder.line_to(iced::Point::new(105.00, 92.00));
        builder.line_to(iced::Point::new(105.37, 89.18));
        builder.line_to(iced::Point::new(106.47, 86.44));
        builder.line_to(iced::Point::new(108.27, 83.83));
        builder.line_to(iced::Point::new(110.73, 81.42));
        builder.line_to(iced::Point::new(113.79, 79.27));
        builder.line_to(iced::Point::new(117.37, 77.44));
        builder.line_to(iced::Point::new(121.38, 75.96));
        builder.line_to(iced::Point::new(125.73, 74.88));
        builder.line_to(iced::Point::new(130.31, 74.22));
        builder.line_to(iced::Point::new(135.00, 74.00));
        builder.line_to(iced::Point::new(139.69, 74.22));
        builder.line_to(iced::Point::new(144.27, 74.88));
        builder.line_to(iced::Point::new(148.62, 75.96));
        builder.line_to(iced::Point::new(152.63, 77.44));
        builder.line_to(iced::Point::new(156.21, 79.27));
        builder.line_to(iced::Point::new(159.27, 81.42));
        builder.line_to(iced::Point::new(161.73, 83.83));
        builder.line_to(iced::Point::new(163.53, 86.44));
        builder.line_to(iced::Point::new(164.63, 89.18));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.05, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(147.00, 92.00));
        builder.line_to(iced::Point::new(146.85, 93.10));
        builder.line_to(iced::Point::new(146.41, 94.16));
        builder.line_to(iced::Point::new(145.69, 95.18));
        builder.line_to(iced::Point::new(144.71, 96.11));
        builder.line_to(iced::Point::new(143.49, 96.95));
        builder.line_to(iced::Point::new(142.05, 97.66));
        builder.line_to(iced::Point::new(140.45, 98.24));
        builder.line_to(iced::Point::new(138.71, 98.66));
        builder.line_to(iced::Point::new(136.88, 98.91));
        builder.line_to(iced::Point::new(135.00, 99.00));
        builder.line_to(iced::Point::new(133.12, 98.91));
        builder.line_to(iced::Point::new(131.29, 98.66));
        builder.line_to(iced::Point::new(129.55, 98.24));
        builder.line_to(iced::Point::new(127.95, 97.66));
        builder.line_to(iced::Point::new(126.51, 96.95));
        builder.line_to(iced::Point::new(125.29, 96.11));
        builder.line_to(iced::Point::new(124.31, 95.18));
        builder.line_to(iced::Point::new(123.59, 94.16));
        builder.line_to(iced::Point::new(123.15, 93.10));
        builder.line_to(iced::Point::new(123.00, 92.00));
        builder.line_to(iced::Point::new(123.15, 90.90));
        builder.line_to(iced::Point::new(123.59, 89.84));
        builder.line_to(iced::Point::new(124.31, 88.82));
        builder.line_to(iced::Point::new(125.29, 87.89));
        builder.line_to(iced::Point::new(126.51, 87.05));
        builder.line_to(iced::Point::new(127.95, 86.34));
        builder.line_to(iced::Point::new(129.55, 85.76));
        builder.line_to(iced::Point::new(131.29, 85.34));
        builder.line_to(iced::Point::new(133.12, 85.09));
        builder.line_to(iced::Point::new(135.00, 85.00));
        builder.line_to(iced::Point::new(136.88, 85.09));
        builder.line_to(iced::Point::new(138.71, 85.34));
        builder.line_to(iced::Point::new(140.45, 85.76));
        builder.line_to(iced::Point::new(142.05, 86.34));
        builder.line_to(iced::Point::new(143.49, 87.05));
        builder.line_to(iced::Point::new(144.71, 87.89));
        builder.line_to(iced::Point::new(145.69, 88.82));
        builder.line_to(iced::Point::new(146.41, 89.84));
        builder.line_to(iced::Point::new(146.85, 90.90));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.70, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(185.00, 110.00));
        builder.line_to(iced::Point::new(195.00, 116.00));
        builder.line_to(iced::Point::new(200.00, 112.00));
        builder.line_to(iced::Point::new(190.00, 106.00));
        builder.close();
    });
    frame.fill(&path, darkest_blue);

}

/// Draw the vectorized floppy (unselected)
pub fn draw_unselected(frame: &mut canvas::Frame) {
    let red = Color::new(0.953, 0.204, 0.204, 1.0);
    let darkest_blue = Color::new(0.055, 0.055, 0.090, 1.0);

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(138.00, 35.00));
        builder.line_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.line_to(iced::Point::new(20.00, 141.00));
        builder.close();
    });
    frame.fill(&path, darkest_blue);

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(102.00, 185.00));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.line_to(iced::Point::new(20.00, 141.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.02, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(102.00, 185.00));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(102.00, 192.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.15, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(208.20, 102.90));
        builder.line_to(iced::Point::new(211.80, 87.90));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.15, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(20.00, 134.00));
        builder.line_to(iced::Point::new(138.00, 35.00));
        builder.line_to(iced::Point::new(211.80, 80.90));
        builder.line_to(iced::Point::new(208.20, 95.90));
        builder.line_to(iced::Point::new(102.00, 185.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.30, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(27.00, 132.32));
        builder.line_to(iced::Point::new(136.74, 40.25));
        builder.line_to(iced::Point::new(205.37, 82.94));
        builder.line_to(iced::Point::new(202.03, 96.89));
        builder.line_to(iced::Point::new(103.26, 179.75));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.02, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(62.12, 102.86));
        builder.line_to(iced::Point::new(95.04, 75.24));
        builder.line_to(iced::Point::new(127.83, 95.63));
        builder.line_to(iced::Point::new(94.91, 123.25));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.30, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(38.00, 142.00));
        builder.line_to(iced::Point::new(94.00, 176.00));
        builder.line_to(iced::Point::new(137.00, 140.00));
        builder.line_to(iced::Point::new(81.00, 106.00));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.30, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(165.00, 92.00));
        builder.line_to(iced::Point::new(164.63, 94.82));
        builder.line_to(iced::Point::new(163.53, 97.56));
        builder.line_to(iced::Point::new(161.73, 100.17));
        builder.line_to(iced::Point::new(159.27, 102.58));
        builder.line_to(iced::Point::new(156.21, 104.73));
        builder.line_to(iced::Point::new(152.63, 106.56));
        builder.line_to(iced::Point::new(148.62, 108.04));
        builder.line_to(iced::Point::new(144.27, 109.12));
        builder.line_to(iced::Point::new(139.69, 109.78));
        builder.line_to(iced::Point::new(135.00, 110.00));
        builder.line_to(iced::Point::new(130.31, 109.78));
        builder.line_to(iced::Point::new(125.73, 109.12));
        builder.line_to(iced::Point::new(121.38, 108.04));
        builder.line_to(iced::Point::new(117.37, 106.56));
        builder.line_to(iced::Point::new(113.79, 104.73));
        builder.line_to(iced::Point::new(110.73, 102.58));
        builder.line_to(iced::Point::new(108.27, 100.17));
        builder.line_to(iced::Point::new(106.47, 97.56));
        builder.line_to(iced::Point::new(105.37, 94.82));
        builder.line_to(iced::Point::new(105.00, 92.00));
        builder.line_to(iced::Point::new(105.37, 89.18));
        builder.line_to(iced::Point::new(106.47, 86.44));
        builder.line_to(iced::Point::new(108.27, 83.83));
        builder.line_to(iced::Point::new(110.73, 81.42));
        builder.line_to(iced::Point::new(113.79, 79.27));
        builder.line_to(iced::Point::new(117.37, 77.44));
        builder.line_to(iced::Point::new(121.38, 75.96));
        builder.line_to(iced::Point::new(125.73, 74.88));
        builder.line_to(iced::Point::new(130.31, 74.22));
        builder.line_to(iced::Point::new(135.00, 74.00));
        builder.line_to(iced::Point::new(139.69, 74.22));
        builder.line_to(iced::Point::new(144.27, 74.88));
        builder.line_to(iced::Point::new(148.62, 75.96));
        builder.line_to(iced::Point::new(152.63, 77.44));
        builder.line_to(iced::Point::new(156.21, 79.27));
        builder.line_to(iced::Point::new(159.27, 81.42));
        builder.line_to(iced::Point::new(161.73, 83.83));
        builder.line_to(iced::Point::new(163.53, 86.44));
        builder.line_to(iced::Point::new(164.63, 89.18));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.02, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(147.00, 92.00));
        builder.line_to(iced::Point::new(146.85, 93.10));
        builder.line_to(iced::Point::new(146.41, 94.16));
        builder.line_to(iced::Point::new(145.69, 95.18));
        builder.line_to(iced::Point::new(144.71, 96.11));
        builder.line_to(iced::Point::new(143.49, 96.95));
        builder.line_to(iced::Point::new(142.05, 97.66));
        builder.line_to(iced::Point::new(140.45, 98.24));
        builder.line_to(iced::Point::new(138.71, 98.66));
        builder.line_to(iced::Point::new(136.88, 98.91));
        builder.line_to(iced::Point::new(135.00, 99.00));
        builder.line_to(iced::Point::new(133.12, 98.91));
        builder.line_to(iced::Point::new(131.29, 98.66));
        builder.line_to(iced::Point::new(129.55, 98.24));
        builder.line_to(iced::Point::new(127.95, 97.66));
        builder.line_to(iced::Point::new(126.51, 96.95));
        builder.line_to(iced::Point::new(125.29, 96.11));
        builder.line_to(iced::Point::new(124.31, 95.18));
        builder.line_to(iced::Point::new(123.59, 94.16));
        builder.line_to(iced::Point::new(123.15, 93.10));
        builder.line_to(iced::Point::new(123.00, 92.00));
        builder.line_to(iced::Point::new(123.15, 90.90));
        builder.line_to(iced::Point::new(123.59, 89.84));
        builder.line_to(iced::Point::new(124.31, 88.82));
        builder.line_to(iced::Point::new(125.29, 87.89));
        builder.line_to(iced::Point::new(126.51, 87.05));
        builder.line_to(iced::Point::new(127.95, 86.34));
        builder.line_to(iced::Point::new(129.55, 85.76));
        builder.line_to(iced::Point::new(131.29, 85.34));
        builder.line_to(iced::Point::new(133.12, 85.09));
        builder.line_to(iced::Point::new(135.00, 85.00));
        builder.line_to(iced::Point::new(136.88, 85.09));
        builder.line_to(iced::Point::new(138.71, 85.34));
        builder.line_to(iced::Point::new(140.45, 85.76));
        builder.line_to(iced::Point::new(142.05, 86.34));
        builder.line_to(iced::Point::new(143.49, 87.05));
        builder.line_to(iced::Point::new(144.71, 87.89));
        builder.line_to(iced::Point::new(145.69, 88.82));
        builder.line_to(iced::Point::new(146.41, 89.84));
        builder.line_to(iced::Point::new(146.85, 90.90));
        builder.close();
    });
    frame.fill(&path, Color { a: 0.30, ..red });

    let path = canvas::Path::new(|builder| {
        builder.move_to(iced::Point::new(185.00, 110.00));
        builder.line_to(iced::Point::new(195.00, 116.00));
        builder.line_to(iced::Point::new(200.00, 112.00));
        builder.line_to(iced::Point::new(190.00, 106.00));
        builder.close();
    });
    frame.fill(&path, darkest_blue);

}