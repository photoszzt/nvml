// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone)]
#[repr(C)]
pub struct PersistentObject<T: Persistable>
{
	oid: PMEMoid,
	phantomData: PhantomData<T>
}

impl<T: Persistable> PartialOrd for PersistentObject<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &PersistentObject<T>) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl<T: Persistable> Ord for PersistentObject<T>
{
	#[inline(always)]
	fn cmp(&self, other: &PersistentObject<T>) -> Ordering
	{
		let ourOid = self.oid;
		let otherOid = other.oid;
		ourOid.pool_uuid_lo.cmp(&otherOid.pool_uuid_lo).then_with(|| ourOid.off.cmp(&otherOid.off))
	}
}

impl<T: Persistable> PartialEq for PersistentObject<T>
{
	#[inline(always)]
	fn eq(&self, other: &PersistentObject<T>) -> bool
	{
		self.oid.equals(&other.oid)
	}
}

impl<T: Persistable> Eq for PersistentObject<T>
{
}

impl<T: Persistable> Hash for PersistentObject<T>
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.oid.pool_uuid_lo.hash(state);
		self.oid.off.hash(state);
	}
}

impl<T: Persistable> Debug for PersistentObject<T>
{
	#[inline(always)]
	default fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		if unlikely(self.is_null())
		{
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.typeNumber())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, OID({}, {}))", T::TypeNumber, self.typeNumber(), self.oid.pool_uuid_lo, self.oid.off)
		}
	}
}

impl<T: Persistable> Debug for PersistentObject<T>
where T: Debug
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		if unlikely(self.is_null())
		{
			write!(f, "PersistentObject({}, {}, NULL)", T::TypeNumber, self.typeNumber())
		}
		else
		{
			write!(f, "PersistentObject({}, {}, {:?})", T::TypeNumber, self.typeNumber(), self.deref())
		}
	}
}

impl<T: Persistable> Display for PersistentObject<T>
where T: Display
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error>
	{
		if unlikely(self.is_null())
		{
			write!(f, "NULL")
		}
		else
		{
			self.deref().fmt(f)
		}
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> Deref for PersistentObject<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &*self.as_ptr() }
	}
}

/// It is possible to violate aliasing rules
impl<T: Persistable> DerefMut for PersistentObject<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut T
	{
		debug_assert!(!self.oid.is_null(), "oid is null");
		
		unsafe { &mut *self.as_ptr() }
	}
}

impl<T: Persistable> Iterator for PersistentObject<T>
{
	type Item = PersistentObject<T>;
	
	fn next(&mut self) -> Option<PersistentObject<T>>
	{
		loop
		{
			// pmemobj_next is safe if self.oid.is_null()
			let next = unsafe { pmemobj_next(self.oid) };
			if unlikely(next.is_null())
			{
				return None;
			}
			// Not necessarily true, but if we're treating a pool as a vectored list then we ought to optimise for this branch
			if likely(next.typeNumber() == T::TypeNumber)
			{
				return Some(PersistentObject::new(self.oid));
			}
		}
	}
}

impl<T: Persistable> OID for PersistentObject<T>
{
	#[inline(always)]
	fn is_null(&self) -> bool
	{
		self.oid.is_null()
	}
	
	#[inline(always)]
	fn equals(&self, other: &Self) -> bool
	{
		self.oid.equals(&other.oid)
	}
	
	#[inline(always)]
	fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		let objectPool = self.oid.persistentObjectPool();
		debug_assert!(!objectPool.is_null(), "How is the objectPool null for an allocated object?");
		objectPool
	}
	
	#[inline(always)]
	fn allocatedUsefulSize(&self) -> size_t
	{
		self.oid.allocatedUsefulSize()
	}
	
	#[inline(always)]
	fn typeNumber(&self) -> TypeNumber
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		self.oid.typeNumber()
	}
	
	#[inline(always)]
	fn address(&self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "Null; unallocated");
		
		let address = self.oid.address();
		debug_assert!(!address.is_null(), "How is the address null for an allocated object?");
		address
	}
}

impl<T: Persistable> PersistentObject<T>
where T: ReadWriteLockablePersistable
{
	#[inline(always)]
	fn readWriteLock<'a>(&'a mut self) -> ReadWriteLock<'a, T>
	{
		let persistentObjectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		ReadWriteLock::new(persistentObjectPool, object.pmemReadWriteLock(), object)
	}
	
	#[inline(always)]
	pub fn readLock<'a>(&'a mut self) -> ReadLockUnlock<'a, T>
	{
		self.readWriteLock().readLock()
	}
	
	#[inline(always)]
	pub fn tryReadLock<'a>(&'a mut self) -> Option<ReadLockUnlock<'a, T>>
	{
		self.readWriteLock().tryReadLock()
	}
	
	#[inline(always)]
	pub fn timedReadLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<ReadLockUnlock<'a, T>>
	{
		self.readWriteLock().timedReadLock(absoluteTimeOut)
	}
	
	#[inline(always)]
	pub fn writeLockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.readWriteLock().writeLockInTransaction(transaction)
	}
	
	#[inline(always)]
	pub fn writeLock<'a>(&'a mut self) -> WriteLockUnlock<'a, T>
	{
		self.readWriteLock().writeLock()
	}
	
	#[inline(always)]
	pub fn tryWriteLock<'a>(&'a mut self) -> Option<WriteLockUnlock<'a, T>>
	{
		self.readWriteLock().tryWriteLock()
	}
	
	#[inline(always)]
	pub fn timedWriteLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<WriteLockUnlock<'a, T>>
	{
		self.readWriteLock().timedWriteLock(absoluteTimeOut)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: MutexLockablePersistable
{
	#[inline(always)]
	fn mutexLock<'a>(&'a mut self) -> MutexLock<'a, T>
	{
		let persistentObjectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		MutexLock::new(persistentObjectPool, object.pmemMutex(), object)
	}
	
	#[inline(always)]
	pub fn lockInTransaction<'a>(&'a mut self, transaction: Transaction)
	{
		self.mutexLock().lockInTransaction(transaction)
	}
	
	#[inline(always)]
	pub fn lock<'a>(&'a mut self) -> MutexUnlock<'a, T>
	{
		self.mutexLock().lock()
	}
	
	#[inline(always)]
	pub fn tryLock<'a>(&'a mut self) -> Option<MutexUnlock<'a, T>>
	{
		self.mutexLock().tryLock()
	}
	
	#[inline(always)]
	pub fn timedLock<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<MutexUnlock<'a, T>>
	{
		self.mutexLock().timedLock(absoluteTimeOut)
	}
}

impl<T: Persistable> PersistentObject<T>
where T: ConditionVariableMutexLockablePersistable
{
	#[inline(always)]
	pub fn lockWithConditionVariable<'a>(&'a mut self) -> (MutexUnlock<'a, T>, ConditionVariable<'a, T>)
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		(mutexLock.lock(), conditionVariable)
	}
	
	#[inline(always)]
	pub fn tryLockWithConditionVariable<'a>(&'a mut self) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		mutexLock.tryLock().map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[inline(always)]
	pub fn timedLockWithConditionVariable<'a>(&'a mut self, absoluteTimeOut: &timespec) -> Option<(MutexUnlock<'a, T>, ConditionVariable<'a, T>)>
	{
		let (mutexLock, conditionVariable) = self.construct();
		
		mutexLock.timedLock(absoluteTimeOut).map(|mutexUnlock| (mutexUnlock, conditionVariable))
	}
	
	#[inline(always)]
	fn construct<'a>(&'a mut self) -> (MutexLock<'a, T>, ConditionVariable<'a, T>)
	{
		let objectPool = self.persistentObjectPool();
		let object = self.deref_mut();
		let conditionVariable = ConditionVariable::new(objectPool, object.pmemConditionVariable());
		let mutexLock = MutexLock::new(objectPool, object.pmemMutex(), object);
		
		(mutexLock, conditionVariable)
	}
}

impl<T: Persistable> PersistentObject<T>
{
	// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
	#[inline(always)]
	pub fn allocateUninitializedAndConstructRootObject(&mut self, objectPool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), GenericError>
	{
		debug_assert!(T::TypeNumber == 0, "This is not a root type, ie type number is '{}'");
		
		#[inline(always)]
		fn allocate<T: Persistable>(objectPool: *mut PMEMobjpool, oidPointer: &mut PMEMoid, constructor: pmemobj_constr, arguments: *mut c_void) -> bool
		{
			let size = T::size();
			debug_assert!(size != 0, "size can not be zero");
			debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
			
			let oid = unsafe { pmemobj_root_construct(objectPool, size, constructor, arguments) };
			*oidPointer = oid;
			oid.is_null()
		}
		
		Self::allocateUninitializedAndConstructInternal(objectPool, &mut self.oid, (allocate::<T>), arguments)
	}
	
	// At this point, self.oid can be garbage; it might also point to an existing object which hasn't been free'd
	#[inline(always)]
	pub fn allocateUninitializedAndConstruct(&mut self, objectPool: *mut PMEMobjpool, arguments: &mut T::Arguments) -> Result<(), GenericError>
	{
		#[inline(always)]
		fn allocate<T: Persistable>(objectPool: *mut PMEMobjpool, oidPointer: &mut PMEMoid, constructor: pmemobj_constr, arguments: *mut c_void) -> bool
		{
			let size = T::size();
			debug_assert!(size != 0, "size can not be zero");
			debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
			
			let result = unsafe { pmemobj_alloc(objectPool, oidPointer, size, T::TypeNumber, constructor, arguments) };
			debug_assert!(result == 0 || result == -1, "result was '{}'", result);
			result == -1
		}
		
		Self::allocateUninitializedAndConstructInternal(objectPool, &mut self.oid, (allocate::<T>), arguments)
	}
	
	#[inline(always)]
	fn allocateUninitializedAndConstructInternal<A: FnOnce(*mut PMEMobjpool, &mut PMEMoid, pmemobj_constr, *mut c_void) -> bool>(objectPool: *mut PMEMobjpool, oid: &mut PMEMoid, allocate: A, arguments: &mut T::Arguments) -> Result<(), GenericError>
	{
		debug_assert!(!objectPool.is_null(), "objectPool is null");
		
		let size = T::size();
		debug_assert!(size != 0, "size can not be zero");
		debug_assert!(size <= PMEMOBJ_MAX_ALLOC_SIZE, "size '{}' exceeds PMEMOBJ_MAX_ALLOC_SIZE '{}'", size, PMEMOBJ_MAX_ALLOC_SIZE);
		
		#[thread_local] static mut CapturedPanic: Option<Box<Any + Send + 'static>> = None;
		
		unsafe extern "C" fn constructor<T: Persistable>(pop: *mut PMEMobjpool, ptr: *mut c_void, arg: *mut c_void) -> c_int
		{
			let result = catch_unwind(AssertUnwindSafe(||
			{
				debug_assert!(!pop.is_null(), "pop is null");
				debug_assert!(!ptr.is_null(), "ptr is null");
				debug_assert!(!arg.is_null(), "arg is null");
				
				T::initialize(ptr as *mut T, pop, &mut *(arg as *mut T::Arguments))
			}));
			
			match result
			{
				Ok(()) => 0,
				
				Err(panicPayload) =>
				{
					CapturedPanic = Some(panicPayload);
					-1
				},
			}
		}
		
		if unlikely(allocate(objectPool, oid, Some(constructor::<T>), arguments as *mut _ as *mut _))
		{
			let osErrorNumber = errno().0;
			match osErrorNumber
			{
				E::ECANCELED =>
				{
					if let Some(capturedPanic) = unsafe { replace(&mut CapturedPanic, None) }
					{
						resume_unwind(capturedPanic);
					}
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_alloc or pmemobj_root_construct"))
				},
				
				_ =>
				{
					debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set and error was '{}'", osErrorNumber);
					
					Err(GenericError::new(osErrorNumber, pmemobj_errormsg, "pmemobj_alloc or pmemobj_root_construct"))
				}
			}
		}
		else
		{
			debug_assert!(unsafe { CapturedPanic.is_none() }, "CapturedPanic was set yet result was 0 (Ok)");
			
			Ok(())
		}
	}
	
	#[inline(always)]
	fn null() -> Self
	{
		Self::new(unsafe { OID_NULL })
	}
	
	#[inline(always)]
	fn new(oid: PMEMoid) -> Self
	{
		Self
		{
			oid: oid,
			phantomData: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn as_ptr(&self) -> *mut T
	{
		self.address() as *mut _
	}
	
	#[inline(always)]
	pub fn persistentObjectPool(&self) -> *mut PMEMobjpool
	{
		let persistentObjectPool = self.oid.persistentObjectPool();
		debug_assert!(!persistentObjectPool.is_null(), "This object does not have a valid OID");
		persistentObjectPool
	}
	
	#[inline(always)]
	pub fn free(&mut self)
	{
		unsafe { pmemobj_free(&mut self.oid) };
		self.oid = unsafe { OID_NULL };
	}
	
	#[inline(always)]
	pub fn freeInTransaction(self, transaction: Transaction) -> c_int
	{
		transaction.free(self.oid)
	}
	
	/// size can be zero
	#[inline(always)]
	pub fn addRangeSnapshotInTransaction(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		
		transaction.addRangeSnapshotInTransaction(self.oid, offset, size)
	}
	
	/// Can only be called from a work() function
	/// If returns !=0 then the transaction will have been aborted; return immediately from work() function
	/// No checks are made for offset or size
	/// size can be zero
	#[inline(always)]
	pub fn addRangeSnapshotInTransactionWithoutFlush(&self, transaction: Transaction, offset: u64, size: size_t) -> c_int
	{
		debug_assert!(offset + size as u64 <= T::size() as u64, "offset '{}' + size '{}' is bigger than our size '{}'", offset, size, T::size());
		
		transaction.addRangeSnapshotInTransactionWithoutFlush(self.oid, offset, size)
	}
}
