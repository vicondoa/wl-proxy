use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_eglstream_controller object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlEglstreamController {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlEglstreamControllerHandler>,
}

struct DefaultHandler;

impl WlEglstreamControllerHandler for DefaultHandler { }

impl ConcreteObject for WlEglstreamController {
    const XML_VERSION: u32 = 2;
    const INTERFACE: ObjectInterface = ObjectInterface::WlEglstreamController;
    const INTERFACE_NAME: &str = "wl_eglstream_controller";
}

impl WlEglstreamController {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlEglstreamControllerHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlEglstreamControllerHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlEglstreamController {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlEglstreamController")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlEglstreamController {
    /// Since when the attach_eglstream_consumer message is available.
    pub const MSG__ATTACH_EGLSTREAM_CONSUMER__SINCE: u32 = 1;

    /// Create server stream and attach consumer
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    #[inline]
    pub fn try_send_attach_eglstream_consumer(
        &self,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            wl_surface,
            wl_resource,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("wl_surface"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("wl_resource"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_eglstream_controller#{}.attach_eglstream_consumer(wl_surface: wl_surface#{}, wl_resource: wl_buffer#{})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0_id,
            arg1_id,
        ]);
        Ok(())
    }

    /// Create server stream and attach consumer
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    #[inline]
    pub fn send_attach_eglstream_consumer(
        &self,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
    ) {
        let res = self.try_send_attach_eglstream_consumer(
            wl_surface,
            wl_resource,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_controller.attach_eglstream_consumer", &e);
        }
    }

    /// Since when the attach_eglstream_consumer_attribs message is available.
    pub const MSG__ATTACH_EGLSTREAM_CONSUMER_ATTRIBS__SINCE: u32 = 2;

    /// Create server stream and attach consumer using attributes
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it using the given attributes.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    /// - `attribs`: Stream consumer attachment attribs
    #[inline]
    pub fn try_send_attach_eglstream_consumer_attribs(
        &self,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
        attribs: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
        ) = (
            wl_surface,
            wl_resource,
            attribs,
        );
        let arg0 = arg0.core();
        let arg1 = arg1.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("wl_surface"))),
            Some(id) => id,
        };
        let arg1_id = match arg1.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("wl_resource"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: u32, arg2: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_eglstream_controller#{}.attach_eglstream_consumer_attribs(wl_surface: wl_surface#{}, wl_resource: wl_buffer#{}, attribs: {})\n", id, arg0, arg1, debug_array(arg2));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1_id, arg2);
        }
        let Some(endpoint) = &self.core.state.server else {
            return Ok(());
        };
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, None);
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0_id,
            arg1_id,
        ]);
        fmt.array(arg2);
        Ok(())
    }

    /// Create server stream and attach consumer using attributes
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it using the given attributes.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    /// - `attribs`: Stream consumer attachment attribs
    #[inline]
    pub fn send_attach_eglstream_consumer_attribs(
        &self,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
        attribs: &[u8],
    ) {
        let res = self.try_send_attach_eglstream_consumer_attribs(
            wl_surface,
            wl_resource,
            attribs,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_controller.attach_eglstream_consumer_attribs", &e);
        }
    }
}

/// A message handler for [`WlEglstreamController`] proxies.
pub trait WlEglstreamControllerHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlEglstreamController>) {
        slf.core.delete_id();
    }

    /// Create server stream and attach consumer
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_attach_eglstream_consumer(
        &mut self,
        slf: &Rc<WlEglstreamController>,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_attach_eglstream_consumer(
            wl_surface,
            wl_resource,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_controller.attach_eglstream_consumer", &e);
        }
    }

    /// Create server stream and attach consumer using attributes
    ///
    /// Creates the corresponding server side EGLStream from the given wl_buffer
    /// and attaches a consumer to it using the given attributes.
    ///
    /// # Arguments
    ///
    /// - `wl_surface`: wl_surface corresponds to the client surface associated with
    ///                         newly created eglstream
    /// - `wl_resource`: wl_resource corresponding to an EGLStream
    /// - `attribs`: Stream consumer attachment attribs
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_attach_eglstream_consumer_attribs(
        &mut self,
        slf: &Rc<WlEglstreamController>,
        wl_surface: &Rc<WlSurface>,
        wl_resource: &Rc<WlBuffer>,
        attribs: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_attach_eglstream_consumer_attribs(
            wl_surface,
            wl_resource,
            attribs,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_controller.attach_eglstream_consumer_attribs", &e);
        }
    }
}

impl ObjectPrivate for WlEglstreamController {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlEglstreamController, version),
            handler: Default::default(),
        })
    }

    fn delete_id(self: Rc<Self>) -> Result<(), (ObjectError, Rc<dyn Object>)> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err((ObjectError(ObjectErrorKind::HandlerBorrowed), self));
        };
        if let Some(handler) = &mut *handler {
            handler.delete_id(&self);
        } else {
            self.core.delete_id();
        }
        Ok(())
    }

    fn handle_request(self: Rc<Self>, client: &Rc<Client>, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            0 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_eglstream_controller#{}.attach_eglstream_consumer(wl_surface: wl_surface#{}, wl_resource: wl_buffer#{})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("wl_surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("wl_resource", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_attach_eglstream_consumer(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_attach_eglstream_consumer(&self, arg0, arg1);
                }
            }
            1 => {
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("wl_surface")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("wl_resource")));
                };
                offset += 1;
                let arg2;
                (arg2, offset) = parse_array(msg, offset, "attribs")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: u32, arg2: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_eglstream_controller#{}.attach_eglstream_consumer_attribs(wl_surface: wl_surface#{}, wl_resource: wl_buffer#{}, attribs: {})\n", client_id, id, arg0, arg1, debug_array(arg2));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlSurface>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("wl_surface", o.core().interface, ObjectInterface::WlSurface)));
                };
                let arg1_id = arg1;
                let Some(arg1) = client.endpoint.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("wl_resource", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg0 = &arg0;
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_attach_eglstream_consumer_attribs(&self, arg0, arg1, arg2);
                } else {
                    DefaultHandler.handle_attach_eglstream_consumer_attribs(&self, arg0, arg1, arg2);
                }
            }
            n => {
                let _ = client;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn handle_event(self: Rc<Self>, server: &Endpoint, msg: &[u32], fds: &mut VecDeque<Rc<OwnedFd>>) -> Result<(), ObjectError> {
        let Some(mut handler) = self.handler.try_borrow_mut() else {
            return Err(ObjectError(ObjectErrorKind::HandlerBorrowed));
        };
        let handler = &mut *handler;
        match msg[1] & 0xffff {
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "attach_eglstream_consumer",
            1 => "attach_eglstream_consumer_attribs",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let _ = id;
        None
    }
}

impl Object for WlEglstreamController {
    fn core(&self) -> &ObjectCore {
        &self.core
    }

    fn unset_handler(&self) {
        self.handler.set(None);
    }

    fn get_handler_any_ref(&self) -> Result<HandlerRef<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerRef::map(borrowed, |handler| &**handler.as_ref().unwrap() as &dyn Any))
    }

    fn get_handler_any_mut(&self) -> Result<HandlerMut<'_, dyn Any>, HandlerAccessError> {
        let borrowed = self.handler.try_borrow_mut().ok_or(HandlerAccessError::AlreadyBorrowed)?;
        if borrowed.is_none() {
            return Err(HandlerAccessError::NoHandler);
        }
        Ok(HandlerMut::map(borrowed, |handler| &mut **handler.as_mut().unwrap() as &mut dyn Any))
    }
}

impl WlEglstreamController {
    /// Since when the present_mode.dont_care enum variant is available.
    pub const ENM__PRESENT_MODE_DONT_CARE__SINCE: u32 = 1;
    /// Since when the present_mode.fifo enum variant is available.
    pub const ENM__PRESENT_MODE_FIFO__SINCE: u32 = 1;
    /// Since when the present_mode.mailbox enum variant is available.
    pub const ENM__PRESENT_MODE_MAILBOX__SINCE: u32 = 1;

    /// Since when the attrib.present_mode enum variant is available.
    pub const ENM__ATTRIB_PRESENT_MODE__SINCE: u32 = 1;
    /// Since when the attrib.fifo_length enum variant is available.
    pub const ENM__ATTRIB_FIFO_LENGTH__SINCE: u32 = 1;
}

/// Stream present mode
///
/// - dont_care: Using this enum will tell the server to make its own
///              decisions regarding present mode.
///
/// - fifo:      Tells the server to use a fifo present mode. The decision to
///              use fifo synchronous is left up to the server.
///
/// - mailbox:   Tells the server to use a mailbox present mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamControllerPresentMode(pub u32);

impl WlEglstreamControllerPresentMode {
    /// Let the Server decide present mode
    pub const DONT_CARE: Self = Self(0);

    /// Use a fifo present mode
    pub const FIFO: Self = Self(1);

    /// Use a mailbox mode
    pub const MAILBOX: Self = Self(2);
}

impl Debug for WlEglstreamControllerPresentMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DONT_CARE => "DONT_CARE",
            Self::FIFO => "FIFO",
            Self::MAILBOX => "MAILBOX",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Stream consumer attachment attributes
///
/// - present_mode: Must be one of wl_eglstream_controller_present_mode. Tells the
///                 server the desired present mode that should be used.
///
/// - fifo_length:  Only valid when the present_mode attrib is provided and its
///                 value is specified as fifo. Tells the server the desired fifo
///                 length to be used when the desired present_mode is fifo.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamControllerAttrib(pub u32);

impl WlEglstreamControllerAttrib {
    /// Tells the server the desired present mode
    pub const PRESENT_MODE: Self = Self(0);

    /// Tells the server the desired fifo length when the desired presenation_mode is fifo.
    pub const FIFO_LENGTH: Self = Self(1);
}

impl Debug for WlEglstreamControllerAttrib {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PRESENT_MODE => "PRESENT_MODE",
            Self::FIFO_LENGTH => "FIFO_LENGTH",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
