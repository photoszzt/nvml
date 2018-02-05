// This file is part of nvml. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of nvml. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml/master/COPYRIGHT.


/// Stored in Volatile Memory
#[derive(Debug)]
pub struct RestartCopyFromAt<'block_meta_data, B: 'block_meta_data + Block>
{
	chain: Chain<'block_meta_data, B>,
	offset: usize,
	block_meta_data_items: &'block_meta_data BlockMetaDataItems<B>,
}

impl<'block_meta_data, B: 'block_meta_data + Block> RestartCopyFromAt<'block_meta_data, B>
{
	#[inline(always)]
	pub fn new(memory_base_pointer: NonNull<u8>, head_of_chains_linked_list: BlockPointer<B>, block_meta_data_items: &'block_meta_data BlockMetaDataItems<B>) -> Self
	{
		debug_assert!(head_of_chains_linked_list.is_not_null(), "head_of_chains_linked_list is null");
		
		Self
		{
			chain: Some
			(
				Chain
				{
					memory_base_pointer,
					block_pointer: head_of_chains_linked_list,
					block_meta_data: head_of_chains_linked_list.expand_to_pointer_to_meta_data(block_meta_data_items),
				}
			),
			offset: 0,
			block_meta_data_items,
		}
	}
	
	#[inline(always)]
	pub fn copy_bytes_from_chains(&mut self, copy_into_address: NonNull<u8>, copy_into_length: usize)
	{
		debug_assert_ne!(self.offset, self.chain.unwrap().capacity(), "offset should never be the chain length");
		
		if copy_into_length == 0
		{
			return;
		}
		
		if self.offset == 0
		{
			self.copy_bytes_from_chains_offset_is_zero(copy_into_address, copy_into_length)
		}
		else
		{
			self.copy_bytes_from_chains_offset(copy_into_address, copy_into_length, self.offset)
		}
	}
	
	#[inline(always)]
	fn copy_bytes_from_chains_offset(&mut self, copy_into_address: NonNull<u8>, copy_into_capacity: usize, offset: usize)
	{
		let copy_from_chain_address = self.data_ptr_offset(offset);
		let remaining_capacity = self.remaining_capacity(offset);
		
		self._copy_bytes_from_chains_inner(copy_into_address, copy_into_capacity, copy_from_chain_address, remaining_capacity, offset)
	}
	
	#[inline(always)]
	fn copy_bytes_from_chains_offset_is_zero(&mut self, copy_into_address: NonNull<u8>, copy_into_capacity: usize)
	{
		let copy_from_chain_address = self.data_ptr();
		let remaining_capacity = self.capacity();
		
		self._copy_bytes_from_chains_inner(copy_into_address, copy_into_capacity, copy_from_chain_address, remaining_capacity, 0)
	}
	
	#[doc(hidden)]
	fn _copy_bytes_from_chains_inner(&mut self, copy_into_address: NonNull<u8>, copy_into_capacity: usize, copy_from_chain_address: NonNull<u8>, remaining_capacity: usize, offset: usize)
	{
		#[inline(always)]
		fn copy(copy_from_chain_address: NonNull<u8>, copy_into_address: NonNull<u8>, capacity: usize)
		{
			unsafe { copy_nonoverlapping(copy_from_chain_address.as_ptr() as *const _, copy_into_address.as_ptr(), capacity) };
		}
		
		copy(copy_from_chain_address, copy_into_address, copy_into_capacity);
		if copy_into_capacity <= remaining_capacity
		{
			self.restart_at_as_nothing_more_to_copy(copy_into_capacity, remaining_capacity, offset)
		}
		else
		{
			self.next_chain();
			
			let (new_copy_into_address, new_copy_into_capacity) = Self::adjust_copy_into_address_and_capacity(copy_into_address, copy_into_capacity, remaining_capacity);
			self.copy_bytes_from_chains_offset_is_zero(new_copy_into_address, new_copy_into_capacity)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn adjust_copy_into_address_and_capacity(copy_into_address: NonNull<u8>, copy_from_capacity: usize, remaining_capacity: usize) -> (NonNull<u8>, usize)
	{
		debug_assert!(copy_from_capacity >= remaining_capacity, "copy_from_capacity is less than remaining_capacity");
		
		(copy_into_address.offset(remaining_capacity), copy_from_capacity - remaining_capacity)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn restart_at_as_nothing_more_to_copy(&mut self, copy_into_capacity: usize, remaining_capacity: usize, offset: usize)
	{
		if copy_into_capacity == remaining_capacity
		{
			self.next_chain()
		}
		else
		{
			self.offset = offset + copy_into_capacity;
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn next_chain(&mut self)
	{
		self.chain.next_chain(self.block_meta_data_items);
		self.offset = 0;
	}
}
