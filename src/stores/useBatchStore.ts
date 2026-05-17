import { create } from 'zustand'

export interface BatchSelection {
  selectedIds: string[]
  isSelectAll: boolean
}

interface BatchState {
  // Raw state
  selection: BatchSelection

  // Actions
  toggleSelection: (id: string) => void
  toggleSelectAll: () => void
  clearSelection: () => void
  setSelectedIds: (ids: string[]) => void
  setSelectAll: (isSelectAll: boolean) => void
  reset: () => void
}

export const useBatchStore = create<BatchState>((set) => ({
  // Initial state
  selection: {
    selectedIds: [],
    isSelectAll: false,
  },

  // Actions
  toggleSelection: (id) =>
    set((state) => {
      const { selectedIds } = state.selection
      const isSelected = selectedIds.includes(id)
      const newSelectedIds = isSelected
        ? selectedIds.filter((i) => i !== id)
        : [...selectedIds, id]
      return {
        selection: {
          ...state.selection,
          selectedIds: newSelectedIds,
          isSelectAll: false,
        },
      }
    }),

  toggleSelectAll: () =>
    set((state) => ({
      selection: {
        selectedIds: state.selection.isSelectAll ? [] : ['all'],
        isSelectAll: !state.selection.isSelectAll,
      },
    })),

  clearSelection: () =>
    set({
      selection: {
        selectedIds: [],
        isSelectAll: false,
      },
    }),

  setSelectedIds: (ids) =>
    set({
      selection: {
        selectedIds: ids,
        isSelectAll: false,
      },
    }),

  setSelectAll: (isSelectAll) =>
    set({
      selection: {
        selectedIds: isSelectAll ? ['all'] : [],
        isSelectAll,
      },
    }),

  reset: () =>
    set({
      selection: {
        selectedIds: [],
        isSelectAll: false,
      },
    }),
}))