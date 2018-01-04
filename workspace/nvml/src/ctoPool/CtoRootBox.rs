// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// CtoRootBox is very similar to CtoBox but:-
/// * does not implement Drop.
/// * can be sent between threads.
// NOTE: CtoRootBox MUST NOT implement Drop or the code in `CtoPool.open()` could fail spectacularly.
pub struct CtoRootBox<T: CtoSafe + Send + Sync>(*mut T);

unsafe impl<T: CtoSafe + Send + Sync> Send for CtoRootBox<T>
{
}

unsafe impl<T: CtoSafe + Send + Sync> Sync for CtoRootBox<T>
{
}

impl<T: CtoSafe + Send + Sync + PartialEq> PartialEq for CtoRootBox<T>
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		PartialEq::eq(&**self, &**other)
	}
	
	#[inline(always)]
	fn ne(&self, other: &Self) -> bool
	{
		PartialEq::ne(&**self, &**other)
	}
}

impl<T: CtoSafe + Send + Sync + Eq> Eq for CtoRootBox<T>
{
}

impl<T: CtoSafe + Send + Sync + PartialOrd> PartialOrd for CtoRootBox<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&**self, &**other)
	}
	
	#[inline(always)]
	fn lt(&self, other: &Self) -> bool
	{
		PartialOrd::lt(&**self, &**other)
	}
	
	#[inline(always)]
	fn le(&self, other: &Self) -> bool
	{
		PartialOrd::le(&**self, &**other)
	}
	
	#[inline(always)]
	fn ge(&self, other: &Self) -> bool
	{
		PartialOrd::ge(&**self, &**other)
	}
	
	#[inline(always)]
	fn gt(&self, other: &Self) -> bool
	{
		PartialOrd::gt(&**self, &**other)
	}
}

impl<T: CtoSafe + Send + Sync + Ord> Ord for CtoRootBox<T>
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

impl<T: CtoSafe + Send + Sync + Hash> Hash for CtoRootBox<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		(**self).hash(state);
	}
}

impl<T: CtoSafe + Send + Sync + Hasher> Hasher for CtoRootBox<T>
{
	#[inline(always)]
	fn finish(&self) -> u64
	{
		(**self).finish()
	}
	
	#[inline(always)]
	fn write(&mut self, bytes: &[u8])
	{
		(**self).write(bytes)
	}
	
	#[inline(always)]
	fn write_u8(&mut self, i: u8)
	{
		(**self).write_u8(i)
	}
	
	#[inline(always)]
	fn write_u16(&mut self, i: u16)
	{
		(**self).write_u16(i)
	}
	
	#[inline(always)]
	fn write_u32(&mut self, i: u32)
	{
		(**self).write_u32(i)
	}
	
	#[inline(always)]
	fn write_u64(&mut self, i: u64)
	{
		(**self).write_u64(i)
	}
	
	//	#[inline(always)]
//	fn write_u128(&mut self, i: u128)
//	{
//		(**self).write_u128(i)
//	}
	
	#[inline(always)]
	fn write_usize(&mut self, i: usize)
	{
		(**self).write_usize(i)
	}
	
	#[inline(always)]
	fn write_i8(&mut self, i: i8)
	{
		(**self).write_i8(i)
	}
	
	#[inline(always)]
	fn write_i16(&mut self, i: i16)
	{
		(**self).write_i16(i)
	}
	
	#[inline(always)]
	fn write_i32(&mut self, i: i32)
	{
		(**self).write_i32(i)
	}
	
	#[inline(always)]
	fn write_i64(&mut self, i: i64)
	{
		(**self).write_i64(i)
	}
	
	//	#[inline(always)]
//	fn write_i128(&mut self, i: i128)
//	{
//		(**self).write_i128(i)
//	}
	
	#[inline(always)]
	fn write_isize(&mut self, i: isize)
	{
		(**self).write_isize(i)
	}
}

impl<T: CtoSafe + Send + Sync + Display> Display for CtoRootBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Display::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe + Send + Sync + Debug> Debug for CtoRootBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self.deref(), f)
	}
}

impl<T: CtoSafe + Send + Sync> Pointer for CtoRootBox<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Pointer::fmt(&self.deref(), f)
	}
}

impl<T: CtoSafe + Send + Sync> Deref for CtoRootBox<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { &*(self.0 as *const _)}
	}
}

impl<T: CtoSafe + Send + Sync> DerefMut for CtoRootBox<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut *self.0 }
	}
}

impl<T: CtoSafe + Send + Sync> Borrow<T> for CtoRootBox<T>
{
	#[inline(always)]
	fn borrow(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe + Send + Sync> BorrowMut<T> for CtoRootBox<T>
{
	#[inline(always)]
	fn borrow_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<T: CtoSafe + Send + Sync> AsRef<T> for CtoRootBox<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &T
	{
		self.deref()
	}
}

impl<T: CtoSafe + Send + Sync> AsMut<T> for CtoRootBox<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut T
	{
		self.deref_mut()
	}
}

impl<T: CtoSafe + Send + Sync> CtoRootBox<T>
{
	#[inline(always)]
	fn as_ptr(this: &Self) -> *mut T
	{
		this.0
	}
}
