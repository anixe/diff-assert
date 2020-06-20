/// Options for displaying diffs.
#[derive(Clone, Copy, Default, Debug)]
pub struct DisplayOptions<'a> {
    /// Sometimes user want's to compare only subslice of a full str. This argument gives
    /// possibility to "move" whole diff to proper offset.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diff_utils::{Comparison, DisplayOptions};
    /// let file_a = (0..1000).map(|i| if i%2 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    /// let file_b = (0..1000).map(|i| if i%5 == 0 { "foo\n" } else { "bar\n" }).collect::<Vec<&str>>();
    ///
    /// let subslice_a = file_a.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    /// let subslice_b = file_b.into_iter().skip(123).take(10).collect::<Vec<&str>>();
    ///
    /// let result = Comparison::new(&subslice_a, &subslice_b).compare().unwrap();
    /// println!("{}", result.display(DisplayOptions { offset: 123, ..Default::default() }));
    /// ```
    ///
    /// Thanks to the `offset` the output will be:
    /// ```ignore
    /// ... ...   @@ -124,10 +124,10 @@
    /// 124 124   bar
    /// 125      -foo
    /// 126 125   bar
    /// 127 126   foo
    /// 128 127   bar
    /// 129      -foo
    /// 130 128   bar
    /// 131      -foo
    ///     129  +bar
    /// 132 130   bar
    /// 133 131   foo
    ///     132  +bar
    ///     133  +bar
    /// ```
    ///
    /// Default value: 0
    pub offset: usize,
    /// Print extra message before writing diff itself.
    /// It is mostly used to specify the filenames
    pub msg_fmt: &'a str,
}
