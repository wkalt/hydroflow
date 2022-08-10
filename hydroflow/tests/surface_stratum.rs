use std::cell::RefCell;
use std::rc::Rc;

use hydroflow::hydroflow_syntax;
use hydroflow::scheduled::graph::Hydroflow;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

// /// Testing an interesting topology: a self-loop which does nothing.
// /// Doesn't compile due to not knowing what type flows through the empty loop.
// #[test]
// pub fn test_loop() {
//     let mut df = hydroflow_syntax! {
//         a = identity::<usize>();
//         a -> a;
//     };
// }

/// Basic difference test, test difference between two one-off iterators.
#[test]
pub fn test_difference_a() {
    let output = <Rc<RefCell<Vec<usize>>>>::default();
    let output_inner = output.clone();

    let mut df: Hydroflow = hydroflow_syntax! {
        a = difference();
        recv_iter([1, 2, 3, 4]) -> [0]a;
        recv_iter([1, 3, 5, 7]) -> [1]a;
        a -> for_each(|x| output_inner.borrow_mut().push(x));
    };
    df.run_available();

    assert_eq!(&[2, 4], &*output.take());
}

/// More complex different test.
/// Take the difference of each epoch of items and subtract the previous epoch's items.
#[test]
pub fn test_difference_b() -> Result<(), SendError<&'static str>> {
    let (inp_send, inp_recv) = mpsc::unbounded_channel::<&'static str>();

    let output = <Rc<RefCell<Vec<&'static str>>>>::default();
    let output_inner = output.clone();

    let mut df: Hydroflow = hydroflow_syntax! {
        a = difference();
        recv_stream(inp_recv) -> [0]a;
        b = a -> tee();
        b[0] -> next_epoch() -> [1]a;
        b[1] -> for_each(|x| output_inner.borrow_mut().push(x));
    };

    println!("{}", df.serde_graph().unwrap().to_mermaid());

    inp_send.send("01")?;
    inp_send.send("02")?;
    inp_send.send("03")?;
    df.run_epoch();
    assert_eq!(&["01", "02", "03"], &*output.take());

    inp_send.send("02")?;
    inp_send.send("11")?;
    inp_send.send("12")?;
    df.run_epoch();
    assert_eq!(&["11", "12"], &*output.take());

    inp_send.send("02")?;
    inp_send.send("11")?;
    inp_send.send("12")?;
    df.run_epoch();
    assert_eq!(&["02"], &*output.take());

    Ok(())
}

#[test]
pub fn test_epoch_loop() {
    let output = <Rc<RefCell<Vec<usize>>>>::default();
    let output_inner = output.clone();

    // Without `next_epoch()` this would be "unsafe" although legal.
    // E.g. it would spin forever in a single infinite tick/epoch.
    let mut df: Hydroflow = hydroflow_syntax! {
        a = merge() -> tee();
        recv_iter([1, 3]) -> [0]a;
        a[0] -> next_epoch() -> map(|x| 2 * x) -> [1]a;
        a[1] -> for_each(|x| output_inner.borrow_mut().push(x));
    };

    df.run_epoch();
    assert_eq!(&[1, 3], &*output.take());

    df.run_epoch();
    assert_eq!(&[2, 6], &*output.take());

    df.run_epoch();
    assert_eq!(&[4, 12], &*output.take());

    df.run_epoch();
    assert_eq!(&[8, 24], &*output.take());
}

// RUSTFLAGS="$RUSTFLAGS -Z proc-macro-backtrace" cargo test --package hydroflow --test surface_stratum -- test_surface_syntax_strata --exact --nocapture
#[test]
pub fn test_surface_syntax_strata() {
    // An edge in the input data = a pair of `usize` vertex IDs.
    let (pairs_send, pairs_recv) = tokio::sync::mpsc::unbounded_channel::<(usize, usize)>();

    let mut df = hydroflow_syntax! {
        reached_vertices = merge() -> map(|v| (v, ()));
        recv_iter(vec![0]) -> [0]reached_vertices;

        edges = recv_stream(pairs_recv) -> tee();

        my_join_tee = join() -> map(|(_src, ((), dst))| dst) -> map(|x| x) -> map(|x| x) -> tee();
        reached_vertices -> [0]my_join_tee;
        edges[1] -> [1]my_join_tee;

        my_join_tee[0] -> [1]reached_vertices;

        diff = difference() -> for_each(|x| println!("Not reached: {}", x));

        edges[0] -> flat_map(|(a, b)| [a, b]) -> [0]diff;
        my_join_tee[1] -> [1]diff;
    };

    println!(
        "{}",
        df.serde_graph()
            .expect("No graph found, maybe failed to parse.")
            .to_mermaid()
    );
    df.run_available();

    println!("A");

    pairs_send.send((0, 1)).unwrap();
    pairs_send.send((2, 4)).unwrap();
    pairs_send.send((3, 5)).unwrap();
    pairs_send.send((1, 2)).unwrap();
    df.run_available();

    // println!("B");

    // pairs_send.send((0, 3)).unwrap();
    // df.run_available();
}