//! Typed tests
//!
//! Simulate 1-layer Toy I/O < [u8] > HTTP State Machine < HttpStream > App
//!
//! First bytes based input gets convereted into HTTP Stream which then
/// app consumes it and returns a response converted back to bytes

use super::*;

use core::marker::PhantomData;

#[derive(Debug)]
struct NothingBurger;

#[derive(Debug)]
struct ToyHttpCtx<L, R> {
    l: PhantomData<L>,
    r: PhantomData<R>,
}

#[derive(Debug)]
struct ToyAppCtx<L> {
    l: PhantomData<L>,
}

/// Marker trait for typed Left side
trait ToyIoBuffer: for <'d> TypedLeft<'d, [u8]> {
}

/// Marker trait for typed Right side
trait ToyHttpBuffer: for<'d> TypedRight<HttpStream<'d>> {
}

impl<'s, 'd: 's, Io, Http> TypedOrbit<'s, 'd, [u8], HttpStream<'d>> for ToyHttpCtx<Io, Http>
where
    Io: ToyIoBuffer,
    Http: ToyHttpBuffer
{
    type Position = NothingBurger;
    type Error = NothingBurger;

    fn typed_advance_with<B, L: TypedLeft<'_, [u8]>, R: TypedRight<HttpStream<'d>>>(
        &'s mut self,
        _: &'s mut B,
        l: &'s mut L,
        r: &'s mut R,
    ) -> Result<Self::Position, Self::Error> {
        let (my_in, my_out) = l.left_typed_mut();
        r.add_right_in(&HttpStream::RequestMeta(HttpMethod::Get));
        //        panic!("ToyHttpCtx<Io, Http> my_in = {:?}, my_out = {:?}", my_in, my_out);
        Ok(NothingBurger)
    }
}

impl<'d, Http> TypedOrbit<HttpStream<'d>, HttpStream<'d>> for ToyAppCtx<Http>
where
    Http: ToyHttpBuffer
{
    type Position = NothingBurger;
    type Error = NothingBurger;

    fn typed_advance_with<'s, B, Tl: TypedLeft<'s, HttpStream<'d>>, Tr: TypedRight<HttpStream<'d>>>(
        &mut self,
        _: &mut B,
        l: &mut Tl,
        r: &mut Tr,
    ) -> Result<Self::Position, Self::Error> {
        let (my_in, my_out) = l.left_typed_mut();
        //r.add_right_in(&HttpStream::RequestMeta(HttpMethod::Get));
        panic!("ToyAppCtx<Http> my_in = {:?}, my_out = {:?}", my_in, my_out);
        //todo!()
        Ok(NothingBurger)
    }
}

struct ToyHttpLayer;

impl<Io, Http, Tl: ?Sized, Tr: ?Sized> TypedBluePrint<ToyHttpCtx<Io, Http>, Tl, Tr> for ToyHttpLayer
where
    Io: ToyIoBuffer,
    Http: ToyHttpBuffer,
    ToyHttpCtx<Io, Http>: TypedOrbit<Tl, Tr>
{
    type Config = NothingBurger;
    type Error = NothingBurger;

    fn typed_with_defaults() -> Result<ToyHttpCtx<Io, Http>, Self::Error> {
        Ok(ToyHttpCtx::<Io, Http>{l: PhantomData, r: PhantomData})
    }
    fn typed_with_configuration(_: Self::Config) -> Result<ToyHttpCtx<Io, Http>, Self::Error> {
        Ok(ToyHttpCtx::<Io, Http>{l: PhantomData, r: PhantomData})
    }
}

struct ToyApp;

impl<Http, Tl, Tr> TypedBluePrint<ToyAppCtx<Http>, Tl, Tr> for ToyApp
where
    Http: ToyHttpBuffer,
    ToyAppCtx<Http>: TypedOrbit<Tl, Tr>
{
    type Config = NothingBurger;
    type Error = NothingBurger;

    fn typed_with_defaults() -> Result<ToyAppCtx<Http>, Self::Error> {
        Ok(ToyAppCtx::<Http>{l: PhantomData})
    }
    fn typed_with_configuration(_: Self::Config) -> Result<ToyAppCtx<Http>, Self::Error> {
        Ok(ToyAppCtx::<Http>{l: PhantomData})
    }
}

struct TestIoBuffer {
    in_buf: [u8; 8192],
    in_buf_len: usize,
    out_buf: [u8; 8192],
    out_buf_len: usize,
}

impl Default for TestIoBuffer {
    fn default() -> Self {
        Self { in_buf: [0; 8192], out_buf: [0; 8192], in_buf_len: 0, out_buf_len: 0 }
    }
}

impl ToyIoBuffer for TestIoBuffer {}
impl<'d> TypedLeft<'d, [u8]> for TestIoBuffer {

    fn left_in_blocked(&self) -> bool { todo!() }
    fn set_left_in_blocked(&mut self, _: bool) -> () {}
    fn left_lens(&self) -> (usize, usize) { todo!() }
    fn left_set_lens(&mut self, _: usize, _: usize) -> () { todo!() }
    fn left_typed_meta(&'d mut self) -> &'d mut [u8] { todo!() }
    fn left_typed_mut(&'d mut self) -> (&'d mut [u8], &'d mut [u8]) {
        (&mut self.in_buf[0..self.in_buf_len], &mut self.out_buf)
    }
    fn is_ready(&self) -> bool { todo!() }
    fn set_ready(&mut self, _: bool) -> bool { todo!() }
    fn left_want_read(&self) -> bool { todo!() }
    fn set_left_want_read(&mut self, _: bool) -> () { todo!() }
    fn left_want_write(&self) -> bool { todo!() }
    fn set_left_want_write(&mut self, _: bool) -> () { todo!() }
    fn shutdown(&mut self) -> () { todo!() }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum HttpMethod {
    #[default]
    Unknown,
    Get,
}

#[derive(Debug)]
struct TestHttpBuffer {
    cur_request: HttpMethod,
    payload_in: Vec<u8>,
    payload_out: Vec<u8>,
}

// This is a naive typed HTTP streaming to test typed L & R
#[derive(Debug)]
enum HttpStream<'d> {
    RequestMeta(HttpMethod),
    Payload(&'d mut [u8]),
}

impl ToyHttpBuffer for TestHttpBuffer {}

impl<'d> TypedLeft<'d, HttpStream<'d>> for TestHttpBuffer {
    fn left_in_blocked(&self) -> bool { todo!() }
    fn set_left_in_blocked(&mut self, _: bool) -> () {}
    fn left_lens(&self) -> (usize, usize) { todo!() }
    fn left_set_lens(&mut self, _: usize, _: usize) -> () { todo!() }
    fn left_typed_meta(&'d mut self) -> &'d mut HttpStream<'d> {
        todo!()
//        HttpStream::Payload(&mut self.payload_out)
    }
    fn left_typed_mut(&'d mut self) -> (&'d mut HttpStream<'d>, &'d mut HttpStream<'d>) {
        todo!()
//        (&mut HttpStream::RequestMeta(self.cur_request), &mut HttpStream::Payload(&mut self.payload_out))
    }
    fn is_ready(&self) -> bool { todo!() }
    fn set_ready(&mut self, _: bool) -> bool { todo!() }
    fn left_want_read(&self) -> bool { todo!() }
    fn set_left_want_read(&mut self, _: bool) -> () { todo!() }
    fn left_want_write(&self) -> bool { todo!() }
    fn set_left_want_write(&mut self, _: bool) -> () { todo!() }
    fn shutdown(&mut self) -> () { todo!() }    
}

impl<'d> TypedRight<HttpStream<'d>> for TestHttpBuffer {

    fn right_lens(&self) -> (usize, usize) { todo!() }
    fn typed_right_out(&self) -> &HttpStream<'d> { todo!() }
    fn wants_right_next_in(&self) -> bool { todo!() }
    fn set_wants_right_next_in(&mut self, _: bool) -> () { todo!() }
    fn all_sent_right_out(&mut self) -> () { todo!() }
    fn add_right_out(&mut self, _: &HttpStream<'d>) -> () { todo!() }
    fn add_right_in(&mut self, a: &HttpStream<'d>) -> () {
        match a {
            HttpStream::RequestMeta(m) => self.cur_request = *m,
            HttpStream::Payload(p) => {
                todo!("add from current.");
            },
        }
    }
}


#[test]
fn typed_http_with_defaults() {
    let http: ToyHttpCtx<TestIoBuffer, TestHttpBuffer> = ToyHttpLayer::typed_with_defaults().unwrap();
}

#[test]
fn typed_app_with_defaults() {
    let app: ToyAppCtx<TestHttpBuffer> = ToyApp::typed_with_defaults().unwrap();
}

#[test]
fn typed_http_with_config() {
    let http: ToyHttpCtx<TestIoBuffer, TestHttpBuffer> = ToyHttpLayer::typed_with_configuration(NothingBurger).unwrap();
}

#[test]
fn typed_app_with_config() {
    let app: ToyAppCtx<TestHttpBuffer> = ToyApp::typed_with_configuration(NothingBurger).unwrap();
}

#[test]
fn typed_http_app_with_config() {

    let mut test_io = TestIoBuffer::default();

    test_io.in_buf[0..18].copy_from_slice("GET / HTTP/1.0\r\n\r\n".as_bytes());
    test_io.in_buf_len = 18;
    
    let mut test_http = TestHttpBuffer { payload_in: Vec::with_capacity(8192), payload_out: Vec::with_capacity(8102), cur_request: Default::default() };

    let mut test_http2 = TestHttpBuffer { payload_in: Vec::with_capacity(8192), payload_out: Vec::with_capacity(8102), cur_request: Default::default() };
    
    let mut http: ToyHttpCtx<TestIoBuffer, TestHttpBuffer> = ToyHttpLayer::typed_with_configuration(NothingBurger).unwrap();
    let mut app: ToyAppCtx<TestHttpBuffer> = ToyApp::typed_with_configuration(NothingBurger).unwrap();

    http.typed_advance_with(&mut NothingBurger, &mut test_io, &mut test_http).unwrap();

    app.typed_advance_with(&mut NothingBurger, &mut test_http, &mut test_http2).unwrap();
    
    assert_eq!(test_http.cur_request, HttpMethod::Get);
    
}
