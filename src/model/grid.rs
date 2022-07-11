use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Grid {
    size: (u32, u32),
    dimensions: (u32, u32),
    names: Vec<Vec<Option<String>>>,
}

impl Grid {
    #[inline]
    pub fn builder() -> GridBuilder {
        GridBuilder {
            size: (129, 129),
            dimensions: (9, 6),
            names: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct GridBuilder {
    size: (u32, u32),
    dimensions: (u32, u32),
    names: Vec<Vec<Option<String>>>,
}

impl GridBuilder {
    #[inline]
    pub fn size(mut self, size: (u32, u32)) -> Self {
        self.size = size;
        self
    }

    #[inline]
    pub fn dimensions(mut self, dimensions: (u32, u32)) -> Self {
        self.dimensions = dimensions;
        self
    }

    #[inline]
    pub fn names<I, J, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = Option<S>>,
        S: Into<String>,
    {
        #[inline]
        fn map_string<S>(option: Option<S>) -> Option<String>
        where
            S: Into<String>,
        {
            option.map(Into::into)
        }

        #[inline]
        fn map_iter<I, S>(iter: I) -> Vec<Option<String>>
        where
            I: IntoIterator<Item = Option<S>>,
            S: Into<String>,
        {
            iter.into_iter().map(map_string).collect()
        }

        let iter = names.into_iter().map(map_iter);

        self.names.extend(iter);
        self
    }

    #[inline]
    pub fn finish(self) -> Grid {
        let Self {
            size,
            dimensions,
            names,
        } = self;

        Grid {
            size,
            dimensions,
            names,
        }
    }
}
