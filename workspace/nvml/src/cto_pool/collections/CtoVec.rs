// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// CTO pool equivalent to a Rust Vec.
pub struct CtoVec<T: CtoSafe>
{
	buf: RawVec<T, CtoPool>,
	len: usize,
}

impl<T: CtoSafe> Drop for CtoVec<T>
{
	// TODO: Review Drop
	fn drop(&mut self)
	{
		unsafe
		{
			// use drop for [T]
			drop_in_place(&mut self[..]);
		}
		// RawVec handles deallocation
	}
}

impl<T: CtoSafe + Clone> Clone for CtoVec<T>
{
	// TODO: Review clone to work with our allocator
	#[inline(always)]
	fn clone(&self) -> Self
	{
		unimplemented!();
	}
}

impl<T: CtoSafe + Hash> Hash for CtoVec<T>
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		Hash::hash(&**self, state)
	}
}

impl<T: CtoSafe + Debug> Debug for CtoVec<T>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(&**self, f)
	}
}

impl<T: CtoSafe + PartialOrd> PartialOrd for CtoVec<T>
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		PartialOrd::partial_cmp(&**self, &**other)
	}
}

impl<T: CtoSafe + Ord> Ord for CtoVec<T>
{
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering
	{
		Ord::cmp(&**self, &**other)
	}
}

macro_rules! __impl_slice_eq1
{
    ($Lhs: ty, $Rhs: ty) =>
   
	{
        __impl_slice_eq1! { $Lhs, $Rhs, Sized }
    };
    ($Lhs: ty, $Rhs: ty, $Bound: ident) =>
   
	{
        impl<'a, 'b, A: $Bound, B: CtoSafe> PartialEq<$Rhs> for $Lhs where A: CtoSafe + PartialEq<B>
       
	{
            #[inline]
            fn eq(&self, other: &$Rhs) -> bool
           
	{
            	self[..] == other[..]
            }
            
            #[inline]
            fn ne(&self, other: &$Rhs) -> bool
           
	{
            	self[..] != other[..]
            }
        }
    }
}

__impl_slice_eq1! { CtoVec<A>, CtoVec<B> }

__impl_slice_eq1! { CtoVec<A>, &'b [B] }

__impl_slice_eq1! { CtoVec<A>, &'b mut [B] }

impl<T: CtoSafe + Eq> Eq for CtoVec<T>
{
}


impl<T: CtoSafe> Deref for CtoVec<T>
{
	type Target = [T];
	
	#[inline(always)]
	fn deref(&self) -> &[T]
	{
		unsafe
		{
			let pointer = self.buf.ptr();
			assume(!pointer.is_null());
			from_raw_parts(pointer, self.len)
		}
	}
}

impl<T: CtoSafe> DerefMut for CtoVec<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut [T]
	{
		unsafe
		{
			let pointer = self.buf.ptr();
			assume(!pointer.is_null());
			from_raw_parts_mut(pointer, self.len)
		}
	}
}

impl<T: CtoSafe> AsRef<CtoVec<T>> for CtoVec<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &Self
	{
		self
	}
}

impl<T: CtoSafe> AsMut<CtoVec<T>> for CtoVec<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Self
	{
		self
	}
}

impl<T: CtoSafe> AsRef<[T]> for CtoVec<T>
{
	#[inline(always)]
	fn as_ref(&self) -> &[T]
	{
		self
	}
}

impl<T: CtoSafe> AsMut<[T]> for CtoVec<T>
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut [T]
	{
		self
	}
}

impl<T: CtoSafe> Index<usize> for CtoVec<T>
{
	type Output = T;
	
	#[inline(always)]
	fn index(&self, index: usize) -> &T
	{
		// NB built-in indexing via `&[T]`
		&(**self)[index]
	}
}

impl<T: CtoSafe> IndexMut<usize> for CtoVec<T>
{
	#[inline]
	fn index_mut(&mut self, index: usize) -> &mut T
	{
		// NB built-in indexing via `&mut [T]`
		&mut (**self)[index]
	}
}

impl<T: CtoSafe> Index<Range<usize>> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, index: Range<usize>) -> &[T]
	{
		Index::index(&**self, index)
	}
}

impl<T: CtoSafe> Index<RangeTo<usize>> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, index: RangeTo<usize>) -> &[T]
	{
		Index::index(&**self, index)
	}
}

impl<T: CtoSafe> Index<RangeFrom<usize>> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, index: RangeFrom<usize>) -> &[T]
	{
		Index::index(&**self, index)
	}
}

impl<T: CtoSafe> Index<RangeFull> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, _index: RangeFull) -> &[T]
	{
		self
	}
}

impl<T: CtoSafe> Index<RangeInclusive<usize>> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, index: RangeInclusive<usize>) -> &[T]
	{
		Index::index(&**self, index)
	}
}

impl<T: CtoSafe> Index<RangeToInclusive<usize>> for CtoVec<T>
{
	type Output = [T];
	
	#[inline]
	fn index(&self, index: RangeToInclusive<usize>) -> &[T]
	{
		Index::index(&**self, index)
	}
}

impl<T: CtoSafe> IndexMut<Range<usize>> for CtoVec<T>
{
	#[inline]
	fn index_mut(&mut self, index: Range<usize>) -> &mut [T]
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T: CtoSafe> IndexMut<RangeTo<usize>> for CtoVec<T>
{
	#[inline]
	fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T]
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T: CtoSafe> IndexMut<RangeFrom<usize>> for CtoVec<T>
{
	#[inline]
	fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T]
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T: CtoSafe> IndexMut<RangeFull> for CtoVec<T>
{
	#[inline]
	fn index_mut(&mut self, _index: RangeFull) -> &mut [T]
	{
		self
	}
}

impl<T: CtoSafe> IndexMut<RangeInclusive<usize>> for CtoVec<T>
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T]
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T: CtoSafe> IndexMut<RangeToInclusive<usize>> for CtoVec<T>
{
	#[inline(always)]
	fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut [T]
	{
		IndexMut::index_mut(&mut **self, index)
	}
}

impl<T: CtoSafe> CtoVec<T>
{
	/// Constructs a new, empty `CtoVec<T, Root>`.
	#[inline(always)]
	pub fn new(cto_pool: &CtoPool) -> Self
	{
		Self
		{
			buf: RawVec::new_in(cto_pool.clone()),
			len: 0,
		}
	}
	
	/// Constructs a new, empty `CtoVec<T>` with the specified capacity.
	#[inline(always)]
	pub fn with_capacity(capacity: usize, cto_pool: &CtoPool) -> Self
	{
		Self
		{
			buf: RawVec::with_capacity_in(capacity, cto_pool.clone()),
			len: 0,
		}
	}
	
	/// Creates a `CtoVec<T>` directly from the raw components of another vector.
	#[inline(always)]
	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize, cto_pool: &CtoPool) -> Self
	{
		Self
		{
			buf: RawVec::from_raw_parts_in(ptr, capacity, cto_pool.clone()),
			len: length,
		}
	}
	
	/// Returns the number of elements the vector can hold without reallocating.
	#[inline(always)]
	pub fn capacity(&self) -> usize
	{
		self.buf.cap()
	}
	
	/// Reserves capacity for at least `additional` more elements to be inserted in the given `CtoVec<T>`.
	#[inline(always)]
	pub fn reserve(&mut self, additional: usize)
	{
		self.buf.reserve(self.len, additional);
	}
	
	/// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the given `Vec<T>`.
	#[inline(always)]
	pub fn reserve_exact(&mut self, additional: usize)
	{
		self.buf.reserve_exact(self.len, additional);
	}
	
	/// Shrinks the capacity of the vector as much as possible.
	#[inline(always)]
	pub fn shrink_to_fit(&mut self)
	{
		self.buf.shrink_to_fit(self.len);
	}
	
	/// Shortens the vector, keeping the first `len` elements and dropping the rest.
	#[inline(always)]
	pub fn truncate(&mut self, len: usize)
	{
		unsafe
		{
			// drop any extra elements
			while len < self.len
			{
				// decrement len before the drop_in_place(), so a panic on Drop doesn't re-drop the just-failed value.
				self.len -= 1;
				let len = self.len;
				drop_in_place(self.get_unchecked_mut(len));
			}
		}
	}
	
	/// Extracts a slice containing the entire vector.
    ///
	/// Equivalent to `&s[..]`.
	#[inline(always)]
	pub fn as_slice(&self) -> &[T]
	{
		self
	}
	
	/// Extracts a mutable slice of the entire vector.
    ///
	/// Equivalent to `&mut s[..]`.
	#[inline(always)]
	pub fn as_mut_slice(&mut self) -> &mut [T]
	{
		self
	}
	
	/// Sets the length of a vector.
	#[inline(always)]
	pub unsafe fn set_len(&mut self, len: usize)
	{
		self.len = len;
	}
	
	/// Removes an element from the vector and returns it.
	///
	/// The removed element is replaced by the last element of the vector.
	///
	/// This does not preserve ordering, but is O(1).
	#[inline(always)]
	pub fn swap_remove(&mut self, index: usize) -> T
	{
		let length = self.len();
		self.swap(index, length - 1);
		self.pop().unwrap()
	}
	
	/// Inserts an element at position `index` within the vector, shifting all
	/// elements after it to the right.
	///
	/// # Panics
	///
	/// Panics if `index > len`.
	pub fn insert(&mut self, index: usize, element: T)
	{
		let len = self.len();
		assert!(index <= len);
		
		// space for the new element
		if len == self.buf.cap()
		{
			self.buf.double();
		}
		
		unsafe
		{
			{
				let p = self.as_mut_ptr().offset(index as isize);
				
				// Move everything over to make space. (Duplicating the `index`th element into two consecutive places).
				copy(p, p.offset(1), len - index);
				
				// Write it in, overwriting the first copy of the `index`th element.
				write(p, element);
			}
			self.set_len(len + 1);
		}
	}
	
	/// Removes and returns the element at position `index` within the vector, shifting all elements after it to the left.
	///
	/// # Panics
	///
	/// Panics if `index` is out of bounds.
	pub fn remove(&mut self, index: usize) -> T
	{
		let len = self.len();
		assert!(index < len);
		
		unsafe
		{
			let result;
			{
				// the place we are taking from.
				let ptr = self.as_mut_ptr().offset(index as isize);
				
				// copy it out, unsafely having a copy of the value on the stack and in the vector at the same time.
				result = read(ptr);
				
				// Shift everything down to fill in that spot.
				copy(ptr.offset(1), ptr, len - index - 1);
			}
			self.set_len(len - 1);
			result
		}
	}
	
	/// Retains only the elements specified by the predicate.
	///
	/// In other words, remove all elements `e` such that `f(&e)` returns `false`.
	/// This method operates in place and preserves the order of the retained
	/// elements.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1, 2, 3, 4];
	/// vec.retain(|&x| x%2 == 0);
	/// assert_eq!(vec, [2, 4]);
	/// ```
	pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F)
	{
		let len = self.len();
		let mut del = 0;
		{
			let v = &mut **self;
			
			for i in 0..len
			{
				if !f(&v[i])
				{
					del += 1;
				}
				else if del > 0
				{
					v.swap(i - del, i);
				}
			}
		}
		
		if del > 0
		{
			self.truncate(len - del);
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Removes all but the first of consecutive elements in the vector that resolve to the same key.
	///
	/// If the vector is sorted, this removes all duplicates.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![10, 20, 21, 30, 20];
	///
	/// vec.dedup_by_key(|i| *i / 10);
	///
	/// assert_eq!(vec, [10, 20, 30, 20]);
	/// ```
	#[inline(always)]
	pub fn dedup_by_key<F, K>(&mut self, mut key: F) where F: FnMut(&mut T) -> K, K: PartialEq
	{
		self.dedup_by(|a, b| key(a) == key(b))
	}
	
	//noinspection SpellCheckingInspection
	/// Removes all but the first of consecutive elements in the vector satisfying a given equality relation.
	///
	/// The `same_bucket` function is passed references to two elements from the vector, and returns `true` if the elements compare equal, or `false` if they do not.
	/// The elements are passed in opposite order from their order in the vector, so if `same_bucket(a, b)` returns `true`, `a` is removed.
	///
	/// If the vector is sorted, this removes all duplicates.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
	///
	/// vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
	///
	/// assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
	/// ```
	pub fn dedup_by<F>(&mut self, mut same_bucket: F) where F: FnMut(&mut T, &mut T) -> bool
	{
		unsafe
		{
			// Although we have a mutable reference to `self`, we cannot make
			// *arbitrary* changes. The `same_bucket` calls could panic, so we
			// must ensure that the vector is in a valid state at all time.
			//
			// The way that we handle this is by using swaps; we iterate
			// over all the elements, swapping as we go so that at the end
			// the elements we wish to keep are in the front, and those we
			// wish to reject are at the back. We can then truncate the
			// vector. This operation is still O(n).
			//
			// Example: We start in this state, where `r` represents "next
			// read" and `w` represents "next_write`.
			//
			//           r
			//     +---+---+---+---+---+---+
			//     | 0 | 1 | 1 | 2 | 3 | 3 |
			//     +---+---+---+---+---+---+
			//           w
			//
			// Comparing self[r] against self[w-1], this is not a duplicate, so
			// we swap self[r] and self[w] (no effect as r==w) and then increment both
			// r and w, leaving us with:
			//
			//               r
			//     +---+---+---+---+---+---+
			//     | 0 | 1 | 1 | 2 | 3 | 3 |
			//     +---+---+---+---+---+---+
			//               w
			//
			// Comparing self[r] against self[w-1], this value is a duplicate,
			// so we increment `r` but leave everything else unchanged:
			//
			//                   r
			//     +---+---+---+---+---+---+
			//     | 0 | 1 | 1 | 2 | 3 | 3 |
			//     +---+---+---+---+---+---+
			//               w
			//
			// Comparing self[r] against self[w-1], this is not a duplicate,
			// so swap self[r] and self[w] and advance r and w:
			//
			//                       r
			//     +---+---+---+---+---+---+
			//     | 0 | 1 | 2 | 1 | 3 | 3 |
			//     +---+---+---+---+---+---+
			//                   w
			//
			// Not a duplicate, repeat:
			//
			//                           r
			//     +---+---+---+---+---+---+
			//     | 0 | 1 | 2 | 3 | 1 | 3 |
			//     +---+---+---+---+---+---+
			//                       w
			//
			// Duplicate, advance r. End of vec. Truncate to w.
			
			let ln = self.len();
			if ln <= 1
			{
				return;
			}
			
			// Avoid bounds checks by using raw pointers.
			let p = self.as_mut_ptr();
			let mut r: usize = 1;
			let mut w: usize = 1;
			
			while r < ln
			{
				let p_r = p.offset(r as isize);
				let p_wm1 = p.offset((w - 1) as isize);
				if !same_bucket(&mut *p_r, &mut *p_wm1)
				{
					if r != w
					{
						let p_w = p_wm1.offset(1);
						swap(&mut *p_r, &mut *p_w);
					}
					w += 1;
				}
				r += 1;
			}
			
			self.truncate(w);
		}
	}
	
	/// Appends an element to the back of a collection.
	///
	/// # Panics
	///
	/// Panics if the number of elements in the vector overflows a `usize`.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1, 2];
	/// vec.push(3);
	/// assert_eq!(vec, [1, 2, 3]);
	/// ```
	#[inline(always)]
	pub fn push(&mut self, value: T)
	{
		// This will panic or abort if we would allocate > `isize::MAX bytes` or if the length increment would overflow for zero-sized types.
		if self.len == self.buf.cap()
		{
			self.buf.double();
		}
		
		unsafe
		{
			let end = self.as_mut_ptr().offset(self.len as isize);
			write(end, value);
			self.len += 1;
		}
	}
	
	/// Removes the last element from a vector and returns it, or [`None`] if it is empty.
	///
	/// [`None`]: ../../std/option/enum.Option.html#variant.None
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1, 2, 3];
	/// assert_eq!(vec.pop(), Some(3));
	/// assert_eq!(vec, [1, 2]);
	/// ```
	#[inline(always)]
	pub fn pop(&mut self) -> Option<T>
	{
		if self.len == 0
		{
			None
		}
		else
		{
			unsafe
			{
				self.len -= 1;
				Some(read(self.get_unchecked(self.len())))
			}
		}
	}
	
	/// Moves all the elements of `other` into `Self`, leaving `other` empty.
	///
	/// # Panics
	///
	/// Panics if the number of elements in the vector overflows a `usize`.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1, 2, 3];
	/// let mut vec2 = vec![4, 5, 6];
	/// vec.append(&mut vec2);
	/// assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
	/// assert_eq!(vec2, []);
	/// ```
	#[inline(always)]
	pub fn append(&mut self, other: &mut Self)
	{
		unsafe
		{
			self.append_elements(other.as_slice() as _);
			other.set_len(0);
		}
	}
	
	/// Appends elements to `Self` from other buffer.
	#[inline]
	unsafe fn append_elements(&mut self, other: *const [T])
	{
		let count = (*other).len();
		self.reserve(count);
		let len = self.len();
		copy_nonoverlapping(other as *const T, self.get_unchecked_mut(len), count);
		self.len += count;
	}
	
	// TODO: pub fn drain<R>(&mut self, range: R) -> Drain<T> where R: RangeArgument<usize>
	
	/// Clears the vector, removing all values.
	///
	/// Note that this method has no effect on the allocated capacity of the vector.
	///
	/// # Examples
	///
	/// ```
	/// let mut v = vec![1, 2, 3];
	///
	/// v.clear();
	///
	/// assert!(v.is_empty());
	/// ```
	#[inline(always)]
	pub fn clear(&mut self)
	{
		self.truncate(0)
	}
	
	/// Returns the number of elements in the vector, also referred to as its 'length'.
	///
	/// # Examples
	///
	/// ```
	/// let a = vec![1, 2, 3];
	/// assert_eq!(a.len(), 3);
	/// ```
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.len
	}
	
	/// Returns `true` if the vector contains no elements.
	///
	/// # Examples
	///
	/// ```
	/// let mut v = Vec::new();
	/// assert!(v.is_empty());
	///
	/// v.push(1);
	/// assert!(!v.is_empty());
	/// ```
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.len() == 0
	}
	
	/// Splits the collection into two at the given index.
	///
	/// Returns a newly allocated `Self`. `self` contains elements `[0, at)`, and the returned `Self` contains elements `[at, len)`.
	///
	/// Note that the capacity of `self` does not change.
	///
	/// # Panics
	///
	/// Panics if `at > len`.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1,2,3];
	/// let vec2 = vec.split_off(1);
	/// assert_eq!(vec, [1]);
	/// assert_eq!(vec2, [2, 3]);
	/// ```
	#[inline(always)]
	pub fn split_off(&mut self, at: usize) -> Self
	{
		assert!(at <= self.len(), "`at` out of bounds");
		
		let other_len = self.len - at;
		let mut other = CtoVec::with_capacity(other_len, self.buf.alloc());
		
		unsafe
		{
			self.set_len(at);
			other.set_len(other_len);
			
			copy_nonoverlapping(self.as_ptr().offset(at as isize), other.as_mut_ptr(), other.len());
		}
		
		other
	}
}

impl<T: CtoSafe + PartialEq> CtoVec<T>
{
	/// Removes consecutive repeated elements in the vector.
	///
	/// If the vector is sorted, this removes all duplicates.
	///
	/// # Examples
	///
	/// ```
	/// let mut vec = vec![1, 2, 2, 3, 2];
	///
	/// vec.dedup();
	///
	/// assert_eq!(vec, [1, 2, 3, 2]);
	/// ```
	#[inline(always)]
	pub fn dedup(&mut self)
	{
		self.dedup_by(|a, b| a == b)
	}
	
	/// Removes the first instance of `item` from the vector if the item exists.
	///
	/// # Examples
	///
	/// ```
	/// # #![feature(vec_remove_item)]
	/// let mut vec = vec![1, 2, 3, 1];
	///
	/// vec.remove_item(&1);
	///
	/// assert_eq!(vec, vec![2, 3, 1]);
	/// ```
	pub fn remove_item(&mut self, item: &T) -> Option<T>
	{
		let pos = self.iter().position(|x| *x == *item)?;
		Some(self.remove(pos))
	}
}
