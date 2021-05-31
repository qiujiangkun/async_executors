use
{
	std::{ time::Duration, future::Future },
};

/// Represents the fact that an executor has timer functionality.
///
//  Implementation:
//  - for tokio: use tokio when tokio_time feature is enabled, futures-timer otherwise.
//  - for async-global-executor: use futures-timer.
//  - for glommio: has own timer that can't be turned off.
//  - for bindgen: use futures-timer
//  - for async-std: has a timer that cannot be turned off.
//
//  The trait needs to be available inconditionally, as a library must be able
//  to depend on it without specifying a backend.
//
pub trait Timer
{
	/// Future returned by sleep().
	//
	type SleepFuture: Future<Output=()> + 'static;

	/// Future that resolves after a given duration.
	//
	#[ must_use = "sleep() returns a future, which does nothing unless awaited" ]
	//
	fn sleep( &self, dur: Duration ) -> Self::SleepFuture;
}



/// Helper functions for timers. This is automatically implemented on all executors
/// that implement [Timer].
//
pub trait TimerExt
{

}


impl<T> TimerExt for T where T: Timer
{

}
