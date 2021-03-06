// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Extension trait to make it easier to work with PMEMlogpool.
trait PMEMlogpoolExt
{
	/// Constant to stop walking log.
	const StopWalking: WalkCallbackResult = 0;
	
	/// Constant to continue walking log.
	const ContinueWalking: WalkCallbackResult = 1;
	
	/// Close the log pool.
	#[inline(always)]
	fn close(self);
	
	/// How many bytes are free in the log pool?
	#[inline(always)]
	fn amount_of_usable_space_in_the_log_pool_in_bytes(self) -> usize;
	
	/// Atomically append to the log (roughly equivalent to `write`).
	#[inline(always)]
	fn append_atomically(self, buffer: *const c_void, count: usize) -> Result<(), AppendError>;
	
	/// Atomically append to the log using an `iovec` (roughly equivalent to `writev`).
	#[inline(always)]
	fn append_vector_atomically(self, buffer: *const iovec, count: u31) -> Result<(), AppendError>;
	
	/// Tell the log (?)
	#[inline(always)]
	fn tell(self) -> i64;
	
	/// Rewind the log.
	#[inline(always)]
	fn rewind(self);
	
	/// Walk ('read') the log.
	/// chunk_size may be zero, in which case `for_each_chunk_callback` is called just once.
	/// callback is called for each chunk
	#[inline(always)]
	fn walk(self, chunk_size: usize, for_each_chunk_callback: ForEachChunkCallback, callback_argument: *mut c_void);
}

macro_rules! debug_assert_self_is_not_null
{
	($self: ident) =>
	{
		debug_assert!($self.is_not_null(), "PMEMlogpool (plp) can not be null");
	}
}

macro_rules! debug_assert_buffer_is_not_null
{
	($buffer: ident) =>
	{
		debug_assert!($buffer.is_not_null(), "buffer can not be null");
	}
}

impl PMEMlogpoolExt for *mut PMEMlogpool
{
	#[inline(always)]
	fn close(self)
	{
		unsafe { pmemlog_close(self) }
	}
	
	#[inline(always)]
	fn amount_of_usable_space_in_the_log_pool_in_bytes(self) -> usize
	{
		unsafe { pmemlog_nbyte(self) }
	}
	
	#[inline(always)]
	fn append_atomically(self, buffer: *const c_void, count: usize) -> Result<(), AppendError>
	{
		debug_assert_self_is_not_null!(self);
		debug_assert_buffer_is_not_null!(buffer);
		
		let result = unsafe { pmemlog_append(self, buffer, count) };
		if likely(result != 0)
		{
			Ok(())
		}
		else if unlikely(result != -1)
		{
			panic!("pmemlog_append() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			match errno().0
			{
				ENOSPC => Err(OutOfSpace),
				EROFS => Err(ReadOnly),
				
				// From pthread_rwlock_wrlock
				EINVAL => panic!("pmemlog_append() pthread_rwlock_wrlock() EINVAL (The value specified by rwlock does not refer to an initialized read-write lock object)"),
				EDEADLK => panic!("pmemlog_append() pthread_rwlock_wrlock() EDEADLK (The current thread already owns the read-write lock for writing or reading)"),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
	}
	
	#[inline(always)]
	fn append_vector_atomically(self, buffer: *const iovec, count: u31) -> Result<(), AppendError>
	{
		debug_assert_self_is_not_null!(self);
		debug_assert_buffer_is_not_null!(buffer);
		debug_assert!(count != 0, "count can not be zero");
		debug_assert!(count <= 2_147_483_648, "count '{}' must be less than or equal to 2^31", count);
		
		let result = unsafe { pmemlog_appendv(self, buffer, count as c_int) };
		if likely(result != 0)
		{
			Ok(())
		}
		else if unlikely(result != -1)
		{
			panic!("pmemlog_appendv() return a value which wasn't -1 or 0, but '{}'", result);
		}
		else
		{
			match errno().0
			{
				ENOSPC => Err(OutOfSpace),
				EROFS => Err(ReadOnly),
				
				// From pthread_rwlock_wrlock
				EINVAL => panic!("pmemlog_appendv() pthread_rwlock_wrlock() EINVAL (The value specified by rwlock does not refer to an initialized read-write lock object)"),
				EDEADLK => panic!("pmemlog_appendv() pthread_rwlock_wrlock() EDEADLK (The current thread already owns the read-write lock for writing or reading)"),
				
				unexpected @ _ => panic!("Unexpected error number '{}'", unexpected),
			}
		}
	}
	
	#[inline(always)]
	fn tell(self) -> i64
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_tell(self) }
	}
	
	#[inline(always)]
	fn rewind(self)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_rewind(self) }
	}
	
	#[inline(always)]
	fn walk(self, chunk_size: usize, for_each_chunk_callback: ForEachChunkCallback, callback_argument: *mut c_void)
	{
		debug_assert_self_is_not_null!(self);
		
		unsafe { pmemlog_walk(self, chunk_size, Some(for_each_chunk_callback), callback_argument) }
	}
}
