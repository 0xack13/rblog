pub trait Render {
    fn render<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>;

    fn render_html<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>;
}

impl Render for tide::Response {
    fn render<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>,
    {
        let mut buf = Vec::new();
        call(&mut buf)?;
        self.set_body(buf);
        Ok(())
    }

    fn render_html<Call>(&mut self, call: Call) -> std::io::Result<()>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>,
    {
        self.render(call)?;
        self.set_content_type(tide::http::mime::HTML);
        Ok(())
    }
}

pub trait RenderBuilder {
    fn render<Call>(self, call: Call) -> std::io::Result<tide::ResponseBuilder>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>;

    fn render_html<Call>(self, call: Call) -> std::io::Result<tide::ResponseBuilder>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>;
}

impl RenderBuilder for tide::ResponseBuilder {
    fn render<Call>(self, call: Call) -> std::io::Result<tide::ResponseBuilder>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>,
    {
        let mut buf = Vec::new();
        call(&mut buf)?;
        Ok(self.body(buf))
    }

    fn render_html<Call>(self, call: Call) -> std::io::Result<tide::ResponseBuilder>
    where
        Call: FnOnce(&mut dyn std::io::Write) -> std::io::Result<()>,
    {
        Ok(self.render(call)?.content_type(tide::http::mime::HTML))
    }
}
