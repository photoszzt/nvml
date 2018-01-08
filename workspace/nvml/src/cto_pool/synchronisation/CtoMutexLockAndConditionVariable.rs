// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Simple wrapper type to make it easier to work correctly with condition variables and mutexes in CtoSafe structures.
#[derive(Debug)]
pub struct CtoMutexLockAndConditionVariable<T: CtoSafe>
{
	cto_mutex_lock: CtoMutexLock<T>,
	cto_condition_variable: CtoConditionVariable,
}

unsafe impl<T: CtoSafe> Send for CtoMutexLockAndConditionVariable<T>
{
}

unsafe impl<T: CtoSafe> Sync for CtoMutexLockAndConditionVariable<T>
{
}

impl<T: CtoSafe> UnwindSafe for CtoMutexLockAndConditionVariable<T>
{
}

impl<T: CtoSafe> RefUnwindSafe for CtoMutexLockAndConditionVariable<T>
{
}

impl<T: CtoSafe> CtoSafe for CtoMutexLockAndConditionVariable<T>
{
	#[inline(always)]
	fn reinitialize(&mut self, cto_pool_inner: &Arc<CtoPoolInner>)
	{
		self.cto_mutex_lock.reinitialize(cto_pool_inner);
		self.cto_condition_variable.reinitialize(cto_pool_inner)
	}
}

impl<T: CtoSafe> CtoMutexLockAndConditionVariable<T>
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(cto_mutex_lock: CtoMutexLock<T>, cto_condition_variable: CtoConditionVariable) -> Self
	{
		Self
		{
			cto_mutex_lock,
			cto_condition_variable,
		}
	}
	
	/// Locks the mutex.
	/// Use the resultant object to access wait / notify behaviour of the condition variable.
	#[inline(always)]
	pub fn lock<'mutex>(&'mutex self) -> CtoMutexLockGuardWithConditionVariable<'mutex, T>
	{
		CtoMutexLockGuardWithConditionVariable
		{
			cto_mutex_lock_guard: self.cto_mutex_lock.lock(),
			cto_condition_variable: &self.cto_condition_variable,
		}
	}
	
	/// Returns Some(lock_guard) if could be locked.
	/// Returns None if the lock is held by another.
	/// Use the resultant object to access wait / notify behaviour of the condition variable.
	#[inline(always)]
	pub fn try_lock<'mutex>(&'mutex self) -> Option<CtoMutexLockGuardWithConditionVariable<'mutex, T>>
	{
		self.cto_mutex_lock.try_lock().map(|cto_mutex_lock_guard| CtoMutexLockGuardWithConditionVariable
		{
			cto_mutex_lock_guard,
			cto_condition_variable: &self.cto_condition_variable,
		})
	}
}
