use void::Void;

pub trait Lens<Target> {
    type Focus;

    fn get(&self, item: Target) -> Self::Focus;
}

pub trait Prism<Target> {
    type Focus;
    type Error;

    fn get(&self, item: Target) -> Result<Self::Focus, Self::Error>;
}

impl<T: Lens<Target>, Target> Prism<Target> for T {
    type Focus = <Self as Lens<Target>>::Focus;
    type Error = Void;

    fn get(&self, item: Target) -> Result<Self::Focus, Self::Error> {
        Ok(self.get(item))
    }
}
