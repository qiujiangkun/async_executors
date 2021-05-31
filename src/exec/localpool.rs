use
{
	crate :: { JoinHandle, InnerJh, SpawnHandle, LocalSpawnHandle } ,
	futures_task :: { SpawnError, FutureObj, LocalFutureObj } ,
	futures_util :: { future::{ FutureExt }, task::{ SpawnExt, LocalSpawnExt } } ,
	std :: { time::Duration } ,
	futures_executor :: { LocalSpawner } ,

};


impl<Out: 'static + Send> SpawnHandle<Out> for LocalSpawner
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for LocalSpawner
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();

		self.spawn_local( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}


#[ cfg( feature = "timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "timer", feature = "async_global" ))) ) ]
//
impl crate::Timer for LocalSpawner
{
	type SleepFuture = futures_timer::Delay;

	fn sleep( &self, dur: Duration ) -> Self::SleepFuture
	{
		futures_timer::Delay::new( dur )
	}
}
