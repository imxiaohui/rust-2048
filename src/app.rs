
use graphics::*;
use piston::*;

use board::Board;
use number_renderer::NumberRenderer;
use settings::Settings;

pub struct App<'a> {
    board: Board<'a>,
    number_renderer: Option<NumberRenderer>,
    settings: &'a Settings,

    logo: Option<Texture>,
    comment1: Option<Texture>,
    comment2: Option<Texture>,

    gl: Gl,
}

impl<'a> App<'a> {
    pub fn new(settings: &'a Settings) -> App<'a> {
        App {
            board: Board::new(settings),
            number_renderer: None,
            settings: settings,

            logo: None,
            comment1: None,
            comment2: None,

            gl: Gl::new(),
        }
    }
}

impl<'a> App<'a> {
    fn render_ui(&mut self, c: &Context) {
        // logo
        c.trans(self.settings.board_padding, self.settings.board_padding)
         .image(self.logo.get_ref())
         .rgb(self.settings.text_dark_color[0],
              self.settings.text_dark_color[1],
              self.settings.text_dark_color[2])
         .draw(&mut self.gl);

        c.view()
         .rect(self.settings.best_rect[0],
               self.settings.best_rect[1],
               self.settings.best_rect[2],
               self.settings.best_rect[3])
         .rgba(self.settings.label_color[0],
               self.settings.label_color[1],
               self.settings.label_color[2],
               1.0)
         .fill(&mut self.gl);

        //let comment1_offset_y = self.settings.comment1_offset_y;
        //let comment1 = self.comment1.as_ref().unwrap();
        //self.render_comment(comment1, comment1_offset_y, c);

        //self.render_comment(self.comment2.get_ref(), self.settings.comment2_offset_y, c);
    }

    fn render_comment(&mut self, comment: &Texture, y: f64, c: &Context) {
        let (width, height) = comment.get_size();
        let w = self.settings.window_size[0] as f64 - 2.0 * self.settings.board_padding;
        let h = height as f64 * w / width as f64;
        c.rect(self.settings.board_padding, y, w, h)
         .image(comment)
         .rgb(self.settings.text_dark_color[0],
              self.settings.text_dark_color[1],
              self.settings.text_dark_color[2])
         .draw(&mut self.gl);
    }
}

impl<'a> Game for App<'a> {
    fn load(&mut self) {
        let asset_store = AssetStore::from_folder(self.settings.asset_folder.as_slice());
        self.number_renderer = Some(NumberRenderer::new(&asset_store));

        self.logo = Some(Texture::from_path(&asset_store.path("logo.png").unwrap()).unwrap());
        self.comment1 = Some(Texture::from_path(&asset_store.path("comment1.png").unwrap()).unwrap());
        self.comment2 = Some(Texture::from_path(&asset_store.path("comment2.png").unwrap()).unwrap());
    }

    fn render(&mut self, args: &mut RenderArgs) {
        let ref c = Context::abs(args.width as f64, args.height as f64);

        let bg = c.rgba(self.settings.window_background_color[0], self.settings.window_background_color[1], self.settings.window_background_color[2], 1.0);
        bg.clear(&mut self.gl);

        self.render_ui(c);
        self.board.render(self.number_renderer.get_ref(), c, &mut self.gl);
    }

    fn update(&mut self, args: &mut UpdateArgs) {
        self.board.update(args.dt);
    }

    fn key_press(&mut self, args: &KeyPressArgs) {
        if args.key == keyboard::Left {
            self.board.merge_from_right_to_left();
        }
        if args.key == keyboard::Right {
            self.board.merge_from_left_to_right();
        }
        if args.key == keyboard::Up {
            self.board.merge_from_bottom_to_top();
        }
        if args.key == keyboard::Down {
            self.board.merge_from_top_to_bottom();
        }
        if args.key == keyboard::Space {
            self.board = Board::new(self.settings);
        }
    }
}

