extern crate plotlib;
use plotlib::page::Page;
use plotlib::repr::{Line, Scatter};
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn main() {
    let K = |x: f64, s: f64| (x * x - s * s).exp();
    let f = |x: f64| (x * x).exp();

    let (start, end) = (0., 1f64);
    let num_points = 50;
    let h = (end - start).abs() / f64::from(num_points);
    let x = (0..num_points)
        .map(|x| lerp(start, end, f64::from(x) / f64::from(num_points)))
        .collect::<Vec<_>>();
    let mut y = vec![0.; num_points as usize];

    y[0] = f(x[0]) * (1. - h * K(x[0], x[0]) / 3.).powi(-1);

    let koeff = |i| if i % 2 == 0 { 4. * h / 3. } else { 2. * h / 3. };
    for i in 1..num_points as usize {
        let mut sum = 0.;

        for j in 1..i - 1 {
            sum += koeff(j) * K(x[i], x[j]) * y[j];
        }

        y[i] = ((1. - koeff(i) * K(x[i], x[i])).powi(-1))
            * (f(x[i]) + (h / 3.) * K(x[i], x[0]) * y[0] + sum);
    }

    let yc = y.clone();
    let data = x.iter().cloned().zip(yc).collect::<Vec<_>>();

    let s1: Scatter = Scatter::from_slice(data.as_slice()).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );

    let view = ContinuousView::new()
        .add(s1)
        .x_label("Йекс")
        .y_label("Уигрек");

    Page::single(&view).save("scatter3.svg").unwrap();

    let func = |x: f64| (x * x + x).exp();
    let size = x.len();
    let dev = (0..size).map(|i| 100. * (((y[i] - func(x[i])) / func(x[i])).abs()));
    let data = x.iter().cloned().zip(dev).collect::<Vec<_>>();

    let s1: Scatter = Scatter::from_slice(data.as_slice()).style(
        PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );

    let l1: Line = Line::new(data).style(plotlib::style::LineStyle::new().colour("#DD3355"));

    let view = ContinuousView::new()
        .add(s1)
        .add(l1)
        .x_label("Йекс")
        .y_label("Уигрек");

    Page::single(&view).save("scatter.svg").unwrap();
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    let t = t.max(0.).min(1.);
    a + (b - a).abs() * t
}
