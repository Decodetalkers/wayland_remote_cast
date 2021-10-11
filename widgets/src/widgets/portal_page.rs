use super::{Notification, NotificationKind};
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

pub trait PortalPageImpl: BinImpl {}

mod imp {
    use gtk::CompositeTemplate;

    use super::*;

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(file = "portal_page.ui")]
    pub struct PortalPage {
        #[template_child]
        pub notification: TemplateChild<Notification>,
        #[template_child]
        pub container: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PortalPage {
        const NAME: &'static str = "PortalPage";
        type Type = super::PortalPage;
        type ParentType = adw::Bin;
        type Interfaces = (gtk::Buildable,);

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_css_name("portal-page");
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for PortalPage {}
    impl WidgetImpl for PortalPage {
        fn unmap(&self, widget: &Self::Type) {
            self.notification.close();

            self.parent_unmap(widget);
        }
    }
    impl BinImpl for PortalPage {}
    impl BuildableImpl for PortalPage {
        fn add_child(
            &self,
            buildable: &Self::Type,
            builder: &gtk::Builder,
            child: &glib::Object,
            type_: Option<&str>,
        ) {
            if buildable.first_child().is_none() {
                self.parent_add_child(buildable, builder, child, type_);
            } else {
                // We first check if the main child `box_` has already been bound.
                self.container
                    .append(child.downcast_ref::<gtk::Widget>().unwrap());
            }
        }
    }
}

glib::wrapper! {
    pub struct PortalPage(ObjectSubclass<imp::PortalPage>) @extends gtk::Widget, adw::Bin, @implements gtk::Buildable;
}

impl PortalPage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a PortalPage")
    }
}

pub trait PortalPageExt {
    fn send_notification(&self, message: &str, kind: NotificationKind);
}

impl<O: IsA<PortalPage>> PortalPageExt for O {
    fn send_notification(&self, message: &str, kind: NotificationKind) {
        let self_ = imp::PortalPage::from_instance(self.as_ref());
        self_.notification.send(message, kind);
    }
}

unsafe impl<T: PortalPageImpl> IsSubclassable<T> for PortalPage {
    fn class_init(class: &mut glib::Class<Self>) {
        <adw::Bin as IsSubclassable<T>>::class_init(class.upcast_ref_mut());
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <adw::Bin as IsSubclassable<T>>::instance_init(instance);
    }
}
