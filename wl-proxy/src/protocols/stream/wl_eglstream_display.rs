use crate::protocol_helpers::prelude::*;
use super::super::all_types::*;

/// A wl_eglstream_display object.
///
/// See the documentation of [the module][self] for the interface description.
pub struct WlEglstreamDisplay {
    core: ObjectCore,
    handler: HandlerHolder<dyn WlEglstreamDisplayHandler>,
}

struct DefaultHandler;

impl WlEglstreamDisplayHandler for DefaultHandler { }

impl ConcreteObject for WlEglstreamDisplay {
    const XML_VERSION: u32 = 1;
    const INTERFACE: ObjectInterface = ObjectInterface::WlEglstreamDisplay;
    const INTERFACE_NAME: &str = "wl_eglstream_display";
}

impl WlEglstreamDisplay {
    /// Sets a new handler.
    pub fn set_handler(&self, handler: impl WlEglstreamDisplayHandler) {
        self.set_boxed_handler(Box::new(handler));
    }

    /// Sets a new, already boxed handler.
    pub fn set_boxed_handler(&self, handler: Box<dyn WlEglstreamDisplayHandler>) {
        if self.core.state.destroyed.get() {
            return;
        }
        self.handler.set(Some(handler));
    }
}

impl Debug for WlEglstreamDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WlEglstreamDisplay")
            .field("server_obj_id", &self.core.server_obj_id.get())
            .field("client_id", &self.core.client_id.get())
            .field("client_obj_id", &self.core.client_obj_id.get())
            .finish()
    }
}

impl WlEglstreamDisplay {
    /// Since when the caps message is available.
    pub const MSG__CAPS__SINCE: u32 = 1;

    /// Server capabilities event
    ///
    /// The capabilities event is sent out at wl_eglstream_display binding
    /// time. It allows the server to advertise what features it supports so
    /// clients may know what is safe to be used.
    ///
    /// # Arguments
    ///
    /// - `caps`: Capabilities mask
    #[inline]
    pub fn try_send_caps(
        &self,
        caps: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
        ) = (
            caps,
        );
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_eglstream_display#{}.caps(caps: {})\n", client_id, id, arg0);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            0,
            arg0 as u32,
        ]);
        Ok(())
    }

    /// Server capabilities event
    ///
    /// The capabilities event is sent out at wl_eglstream_display binding
    /// time. It allows the server to advertise what features it supports so
    /// clients may know what is safe to be used.
    ///
    /// # Arguments
    ///
    /// - `caps`: Capabilities mask
    #[inline]
    pub fn send_caps(
        &self,
        caps: i32,
    ) {
        let res = self.try_send_caps(
            caps,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_display.caps", &e);
        }
    }

    /// Since when the swapinterval_override message is available.
    pub const MSG__SWAPINTERVAL_OVERRIDE__SINCE: u32 = 1;

    /// Server Swap interval override event
    ///
    /// The swapinterval_override event is sent out whenever a client requests
    /// a swapinterval setting through swap_interval() and there is an override
    /// in place that will make such request to be ignored.
    /// The swapinterval_override event will provide the override value so
    /// that the client is made aware of it.
    ///
    /// # Arguments
    ///
    /// - `swapinterval`: Server swap interval override value
    /// - `stream`: wl_buffer corresponding to an EGLStream
    #[inline]
    pub fn try_send_swapinterval_override(
        &self,
        swapinterval: i32,
        stream: &Rc<WlBuffer>,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            swapinterval,
            stream,
        );
        let arg1 = arg1.core();
        let core = self.core();
        let client_ref = core.client.borrow();
        let Some(client) = &*client_ref else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoClient));
        };
        let id = core.client_obj_id.get().unwrap_or(0);
        if arg1.client_id.get() != Some(client.endpoint.id) {
            return Err(ObjectError(ObjectErrorKind::ArgNoClientId("stream", client.endpoint.id)));
        }
        let arg1_id = arg1.client_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, client_id: u64, id: u32, arg0: i32, arg1: u32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} <= wl_eglstream_display#{}.swapinterval_override(swapinterval: {}, stream: wl_buffer#{})\n", client_id, id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, client.endpoint.id, id, arg0, arg1_id);
        }
        let endpoint = &client.endpoint;
        if !endpoint.flush_queued.replace(true) {
            self.core.state.add_flushable_endpoint(endpoint, Some(client));
        }
        let mut outgoing_ref = endpoint.outgoing.borrow_mut();
        let outgoing = &mut *outgoing_ref;
        let mut fmt = outgoing.formatter();
        fmt.words([
            id,
            1,
            arg0 as u32,
            arg1_id,
        ]);
        Ok(())
    }

    /// Server Swap interval override event
    ///
    /// The swapinterval_override event is sent out whenever a client requests
    /// a swapinterval setting through swap_interval() and there is an override
    /// in place that will make such request to be ignored.
    /// The swapinterval_override event will provide the override value so
    /// that the client is made aware of it.
    ///
    /// # Arguments
    ///
    /// - `swapinterval`: Server swap interval override value
    /// - `stream`: wl_buffer corresponding to an EGLStream
    #[inline]
    pub fn send_swapinterval_override(
        &self,
        swapinterval: i32,
        stream: &Rc<WlBuffer>,
    ) {
        let res = self.try_send_swapinterval_override(
            swapinterval,
            stream,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_display.swapinterval_override", &e);
        }
    }

    /// Since when the create_stream message is available.
    pub const MSG__CREATE_STREAM__SINCE: u32 = 1;

    /// Create a wl_buffer from the given handle
    ///
    /// Create a wl_buffer corresponding to given handle. The attributes list
    /// may be used to define additional EGLStream connection data (e.g inet
    /// address/port). The server can create its EGLStream handle using the
    /// information encoded in the wl_buffer.
    ///
    /// # Arguments
    ///
    /// - `id`: New ID
    /// - `width`: Stream framebuffer width
    /// - `height`: Stream framebuffer height
    /// - `handle`: Handle for the stream creation
    /// - `r#type`: Handle type
    /// - `attribs`: Stream extra connection attribs
    #[inline]
    pub fn try_send_create_stream(
        &self,
        id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        handle: &Rc<OwnedFd>,
        r#type: i32,
        attribs: &[u8],
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
        ) = (
            id,
            width,
            height,
            handle,
            r#type,
            attribs,
        );
        let arg0_obj = arg0;
        let arg0 = arg0_obj.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        arg0.generate_server_id(arg0_obj.clone())
            .map_err(|e| ObjectError(ObjectErrorKind::GenerateServerId("id", e)))?;
        let arg0_id = arg0.server_obj_id.get().unwrap_or(0);
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: &[u8]) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_eglstream_display#{}.create_stream(id: wl_buffer#{}, width: {}, height: {}, handle: {}, type: {}, attribs: {})\n", id, arg0, arg1, arg2, arg3, arg4, debug_array(arg5));
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1, arg2, arg3.as_raw_fd(), arg4, arg5);
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
        fmt.fds.push_back(arg3.clone());
        fmt.words([
            id,
            0,
            arg0_id,
            arg1 as u32,
            arg2 as u32,
            arg4 as u32,
        ]);
        fmt.array(arg5);
        Ok(())
    }

    /// Create a wl_buffer from the given handle
    ///
    /// Create a wl_buffer corresponding to given handle. The attributes list
    /// may be used to define additional EGLStream connection data (e.g inet
    /// address/port). The server can create its EGLStream handle using the
    /// information encoded in the wl_buffer.
    ///
    /// # Arguments
    ///
    /// - `id`: New ID
    /// - `width`: Stream framebuffer width
    /// - `height`: Stream framebuffer height
    /// - `handle`: Handle for the stream creation
    /// - `r#type`: Handle type
    /// - `attribs`: Stream extra connection attribs
    #[inline]
    pub fn send_create_stream(
        &self,
        id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        handle: &Rc<OwnedFd>,
        r#type: i32,
        attribs: &[u8],
    ) {
        let res = self.try_send_create_stream(
            id,
            width,
            height,
            handle,
            r#type,
            attribs,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_display.create_stream", &e);
        }
    }

    /// Create a wl_buffer from the given handle
    ///
    /// Create a wl_buffer corresponding to given handle. The attributes list
    /// may be used to define additional EGLStream connection data (e.g inet
    /// address/port). The server can create its EGLStream handle using the
    /// information encoded in the wl_buffer.
    ///
    /// # Arguments
    ///
    /// - `width`: Stream framebuffer width
    /// - `height`: Stream framebuffer height
    /// - `handle`: Handle for the stream creation
    /// - `r#type`: Handle type
    /// - `attribs`: Stream extra connection attribs
    #[inline]
    pub fn new_try_send_create_stream(
        &self,
        width: i32,
        height: i32,
        handle: &Rc<OwnedFd>,
        r#type: i32,
        attribs: &[u8],
    ) -> Result<Rc<WlBuffer>, ObjectError> {
        let id = self.core.create_child();
        self.try_send_create_stream(
            &id,
            width,
            height,
            handle,
            r#type,
            attribs,
        )?;
        Ok(id)
    }

    /// Create a wl_buffer from the given handle
    ///
    /// Create a wl_buffer corresponding to given handle. The attributes list
    /// may be used to define additional EGLStream connection data (e.g inet
    /// address/port). The server can create its EGLStream handle using the
    /// information encoded in the wl_buffer.
    ///
    /// # Arguments
    ///
    /// - `width`: Stream framebuffer width
    /// - `height`: Stream framebuffer height
    /// - `handle`: Handle for the stream creation
    /// - `r#type`: Handle type
    /// - `attribs`: Stream extra connection attribs
    #[inline]
    pub fn new_send_create_stream(
        &self,
        width: i32,
        height: i32,
        handle: &Rc<OwnedFd>,
        r#type: i32,
        attribs: &[u8],
    ) -> Rc<WlBuffer> {
        let id = self.core.create_child();
        self.send_create_stream(
            &id,
            width,
            height,
            handle,
            r#type,
            attribs,
        );
        id
    }

    /// Since when the swap_interval message is available.
    pub const MSG__SWAP_INTERVAL__SINCE: u32 = 1;

    /// change the swap interval of an EGLStream consumer
    ///
    /// Set the swap interval for the consumer of the given EGLStream. The swap
    /// interval is silently clamped to the valid range on the server side.
    ///
    /// # Arguments
    ///
    /// - `stream`: wl_buffer corresponding to an EGLStream
    /// - `interval`: new swap interval
    #[inline]
    pub fn try_send_swap_interval(
        &self,
        stream: &Rc<WlBuffer>,
        interval: i32,
    ) -> Result<(), ObjectError> {
        let (
            arg0,
            arg1,
        ) = (
            stream,
            interval,
        );
        let arg0 = arg0.core();
        let core = self.core();
        let Some(id) = core.server_obj_id.get() else {
            return Err(ObjectError(ObjectErrorKind::ReceiverNoServerId));
        };
        let arg0_id = match arg0.server_obj_id.get() {
            None => return Err(ObjectError(ObjectErrorKind::ArgNoServerId("stream"))),
            Some(id) => id,
        };
        #[cfg(feature = "logging")]
        if self.core.state.log {
            #[cold]
            fn log(state: &State, id: u32, arg0: u32, arg1: i32) {
                let (millis, micros) = time_since_epoch();
                let prefix = &state.log_prefix;
                let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      <= wl_eglstream_display#{}.swap_interval(stream: wl_buffer#{}, interval: {})\n", id, arg0, arg1);
                state.log(args);
            }
            log(&self.core.state, id, arg0_id, arg1);
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
            arg1 as u32,
        ]);
        Ok(())
    }

    /// change the swap interval of an EGLStream consumer
    ///
    /// Set the swap interval for the consumer of the given EGLStream. The swap
    /// interval is silently clamped to the valid range on the server side.
    ///
    /// # Arguments
    ///
    /// - `stream`: wl_buffer corresponding to an EGLStream
    /// - `interval`: new swap interval
    #[inline]
    pub fn send_swap_interval(
        &self,
        stream: &Rc<WlBuffer>,
        interval: i32,
    ) {
        let res = self.try_send_swap_interval(
            stream,
            interval,
        );
        if let Err(e) = res {
            log_send("wl_eglstream_display.swap_interval", &e);
        }
    }
}

/// A message handler for [`WlEglstreamDisplay`] proxies.
pub trait WlEglstreamDisplayHandler: Any {
    /// Event handler for wl_display.delete_id messages deleting the ID of this object.
    ///
    /// The default handler forwards the event to the client, if any.
    #[inline]
    fn delete_id(&mut self, slf: &Rc<WlEglstreamDisplay>) {
        slf.core.delete_id();
    }

    /// Server capabilities event
    ///
    /// The capabilities event is sent out at wl_eglstream_display binding
    /// time. It allows the server to advertise what features it supports so
    /// clients may know what is safe to be used.
    ///
    /// # Arguments
    ///
    /// - `caps`: Capabilities mask
    #[inline]
    fn handle_caps(
        &mut self,
        slf: &Rc<WlEglstreamDisplay>,
        caps: i32,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        let res = slf.try_send_caps(
            caps,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_display.caps", &e);
        }
    }

    /// Server Swap interval override event
    ///
    /// The swapinterval_override event is sent out whenever a client requests
    /// a swapinterval setting through swap_interval() and there is an override
    /// in place that will make such request to be ignored.
    /// The swapinterval_override event will provide the override value so
    /// that the client is made aware of it.
    ///
    /// # Arguments
    ///
    /// - `swapinterval`: Server swap interval override value
    /// - `stream`: wl_buffer corresponding to an EGLStream
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_swapinterval_override(
        &mut self,
        slf: &Rc<WlEglstreamDisplay>,
        swapinterval: i32,
        stream: &Rc<WlBuffer>,
    ) {
        if !slf.core.forward_to_client.get() {
            return;
        }
        if let Some(client_id) = slf.core.client_id.get() {
            if let Some(client_id_2) = stream.core().client_id.get() {
                if client_id != client_id_2 {
                    return;
                }
            }
        }
        let res = slf.try_send_swapinterval_override(
            swapinterval,
            stream,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_display.swapinterval_override", &e);
        }
    }

    /// Create a wl_buffer from the given handle
    ///
    /// Create a wl_buffer corresponding to given handle. The attributes list
    /// may be used to define additional EGLStream connection data (e.g inet
    /// address/port). The server can create its EGLStream handle using the
    /// information encoded in the wl_buffer.
    ///
    /// # Arguments
    ///
    /// - `id`: New ID
    /// - `width`: Stream framebuffer width
    /// - `height`: Stream framebuffer height
    /// - `handle`: Handle for the stream creation
    /// - `r#type`: Handle type
    /// - `attribs`: Stream extra connection attribs
    #[inline]
    fn handle_create_stream(
        &mut self,
        slf: &Rc<WlEglstreamDisplay>,
        id: &Rc<WlBuffer>,
        width: i32,
        height: i32,
        handle: &Rc<OwnedFd>,
        r#type: i32,
        attribs: &[u8],
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_create_stream(
            id,
            width,
            height,
            handle,
            r#type,
            attribs,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_display.create_stream", &e);
        }
    }

    /// change the swap interval of an EGLStream consumer
    ///
    /// Set the swap interval for the consumer of the given EGLStream. The swap
    /// interval is silently clamped to the valid range on the server side.
    ///
    /// # Arguments
    ///
    /// - `stream`: wl_buffer corresponding to an EGLStream
    /// - `interval`: new swap interval
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn handle_swap_interval(
        &mut self,
        slf: &Rc<WlEglstreamDisplay>,
        stream: &Rc<WlBuffer>,
        interval: i32,
    ) {
        if !slf.core.forward_to_server.get() {
            return;
        }
        let res = slf.try_send_swap_interval(
            stream,
            interval,
        );
        if let Err(e) = res {
            log_forward("wl_eglstream_display.swap_interval", &e);
        }
    }
}

impl ObjectPrivate for WlEglstreamDisplay {
    fn new(state: &Rc<State>, version: u32) -> Rc<Self> {
        Rc::<Self>::new_cyclic(|slf| Self {
            core: ObjectCore::new(state, slf.clone(), ObjectInterface::WlEglstreamDisplay, version),
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
                let mut offset = 2;
                let Some(&arg0) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("id")));
                };
                offset += 1;
                let Some(&arg1) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("width")));
                };
                offset += 1;
                let Some(&arg2) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("height")));
                };
                offset += 1;
                let Some(&arg4) = msg.get(offset) else {
                    return Err(ObjectError(ObjectErrorKind::MissingArgument("type")));
                };
                offset += 1;
                let arg5;
                (arg5, offset) = parse_array(msg, offset, "attribs")?;
                if offset != msg.len() {
                    return Err(ObjectError(ObjectErrorKind::TrailingBytes));
                }
                let Some(arg3) = fds.pop_front() else {
                    return Err(ObjectError(ObjectErrorKind::MissingFd("handle")));
                };
                let arg1 = arg1 as i32;
                let arg2 = arg2 as i32;
                let arg3 = &arg3;
                let arg4 = arg4 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: &[u8]) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_eglstream_display#{}.create_stream(id: wl_buffer#{}, width: {}, height: {}, handle: {}, type: {}, attribs: {})\n", client_id, id, arg0, arg1, arg2, arg3, arg4, debug_array(arg5));
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1, arg2, arg3.as_raw_fd(), arg4, arg5);
                }
                let arg0_id = arg0;
                let arg0 = WlBuffer::new(&self.core.state, self.core.version);
                arg0.core().set_client_id(client, arg0_id, arg0.clone())
                    .map_err(|e| ObjectError(ObjectErrorKind::SetClientId(arg0_id, "id", e)))?;
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_create_stream(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                } else {
                    DefaultHandler.handle_create_stream(&self, arg0, arg1, arg2, arg3, arg4, arg5);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg1 = arg1 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, client_id: u64, id: u32, arg0: u32, arg1: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}client#{:<4} -> wl_eglstream_display#{}.swap_interval(stream: wl_buffer#{}, interval: {})\n", client_id, id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, client.endpoint.id, msg[0], arg0, arg1);
                }
                let arg0_id = arg0;
                let Some(arg0) = client.endpoint.lookup(arg0_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoClientObject(client.endpoint.id, arg0_id)));
                };
                let Ok(arg0) = (arg0 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = client.endpoint.lookup(arg0_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("stream", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg0 = &arg0;
                if let Some(handler) = handler {
                    (**handler).handle_swap_interval(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_swap_interval(&self, arg0, arg1);
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
            0 => {
                let [
                    arg0,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 12)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_eglstream_display#{}.caps(caps: {})\n", id, arg0);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0);
                }
                if let Some(handler) = handler {
                    (**handler).handle_caps(&self, arg0);
                } else {
                    DefaultHandler.handle_caps(&self, arg0);
                }
            }
            1 => {
                let [
                    arg0,
                    arg1,
                ] = msg[2..] else {
                    return Err(ObjectError(ObjectErrorKind::WrongMessageSize(msg.len() as u32 * 4, 16)));
                };
                let arg0 = arg0 as i32;
                #[cfg(feature = "logging")]
                if self.core.state.log {
                    #[cold]
                    fn log(state: &State, id: u32, arg0: i32, arg1: u32) {
                        let (millis, micros) = time_since_epoch();
                        let prefix = &state.log_prefix;
                        let args = format_args!("[{millis:7}.{micros:03}] {prefix}server      -> wl_eglstream_display#{}.swapinterval_override(swapinterval: {}, stream: wl_buffer#{})\n", id, arg0, arg1);
                        state.log(args);
                    }
                    log(&self.core.state, msg[0], arg0, arg1);
                }
                let arg1_id = arg1;
                let Some(arg1) = server.lookup(arg1_id) else {
                    return Err(ObjectError(ObjectErrorKind::NoServerObject(arg1_id)));
                };
                let Ok(arg1) = (arg1 as Rc<dyn Any>).downcast::<WlBuffer>() else {
                    let o = server.lookup(arg1_id).unwrap();
                    return Err(ObjectError(ObjectErrorKind::WrongObjectType("stream", o.core().interface, ObjectInterface::WlBuffer)));
                };
                let arg1 = &arg1;
                if let Some(handler) = handler {
                    (**handler).handle_swapinterval_override(&self, arg0, arg1);
                } else {
                    DefaultHandler.handle_swapinterval_override(&self, arg0, arg1);
                }
            }
            n => {
                let _ = server;
                let _ = msg;
                let _ = fds;
                let _ = handler;
                return Err(ObjectError(ObjectErrorKind::UnknownMessageId(n)));
            }
        }
        Ok(())
    }

    fn get_request_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "create_stream",
            1 => "swap_interval",
            _ => return None,
        };
        Some(name)
    }

    fn get_event_name(&self, id: u32) -> Option<&'static str> {
        let name = match id {
            0 => "caps",
            1 => "swapinterval_override",
            _ => return None,
        };
        Some(name)
    }
}

impl Object for WlEglstreamDisplay {
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

impl WlEglstreamDisplay {
    /// Since when the cap.stream_fd enum variant is available.
    pub const ENM__CAP_STREAM_FD__SINCE: u32 = 1;
    /// Since when the cap.stream_inet enum variant is available.
    pub const ENM__CAP_STREAM_INET__SINCE: u32 = 1;
    /// Since when the cap.stream_socket enum variant is available.
    pub const ENM__CAP_STREAM_SOCKET__SINCE: u32 = 1;
}

/// wl_eglstream_display capability codes
///
/// This enum values should be used as bit masks.
///
/// - stream_fd:     The server supports EGLStream connections as described
///                  in EGL_KHR_stream_cross_process_fd
///
/// - stream_inet:   The server supports EGLStream inet connections as
///                  described in EGL_NV_stream_socket.
///
/// - stream_socket: The server supports EGLStream unix socket connections
///                  as described in EGL_NV_stream_socket.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WlEglstreamDisplayCap(pub u32);

impl WlEglstreamDisplayCap {
    /// Stream connection with FD
    pub const STREAM_FD: Self = Self(1);

    /// Stream inet connection
    pub const STREAM_INET: Self = Self(2);

    /// Stream unix connection
    pub const STREAM_SOCKET: Self = Self(4);
}

impl Debug for WlEglstreamDisplayCap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::STREAM_FD => "STREAM_FD",
            Self::STREAM_INET => "STREAM_INET",
            Self::STREAM_SOCKET => "STREAM_SOCKET",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}
