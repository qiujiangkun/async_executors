use
{
	crate            :: { JoinHandle, InnerJh, SpawnHandle      } ,
	futures_task     :: { SpawnError, FutureObj                 } ,
	futures_util     :: { future::{ FutureExt }, task::SpawnExt } ,
	futures_executor :: { ThreadPool                            } ,

};


impl<Out: 'static + Send> SpawnHandle<Out> for ThreadPool
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: InnerJh::RemoteHandle( Some(handle) ) } )
	}
}


#[ cfg( feature = "timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "timer", feature = "async_global" ))) ) ]
//
impl crate::Timer for ThreadPool
{
	type SleepFuture = futures_timer::Delay;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		futures_timer::Delay::new( dur )
	}
}