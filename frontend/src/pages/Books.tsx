import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { booksApi, divisionsApi, usersApi } from '../lib/api';
import { PageHeader } from '../components/PageHeader';

export function Books() {
  const queryClient = useQueryClient();
  const [newBookTitle, setNewBookTitle] = useState('');
  const [newBookDivisionId, setNewBookDivisionId] = useState<number | ''>('');
  const [borrowingBookId, setBorrowingBookId] = useState<number | null>(null);
  const [borrowUserId, setBorrowUserId] = useState<number | ''>('');

  const { data: books = [], isLoading: booksLoading, error: booksError } = useQuery({
    queryKey: ['books'],
    queryFn: async () => {
      const response = await booksApi.list();
      return response.data;
    },
  });

  const { data: divisions = [] } = useQuery({
    queryKey: ['divisions'],
    queryFn: async () => {
      const response = await divisionsApi.list();
      return response.data;
    },
  });

  const { data: users = [] } = useQuery({
    queryKey: ['users'],
    queryFn: async () => {
      const response = await usersApi.list();
      return response.data;
    },
  });

  const createBookMutation = useMutation({
    mutationFn: (data: { title: string; division_id: number }) =>
      booksApi.create(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] });
      setNewBookTitle('');
      setNewBookDivisionId('');
    },
    onError: (error: any) => {
      console.error('Failed to create book:', error);
      alert(`Failed to create book: ${error.response?.data?.message || error.message}`);
    },
  });

  const borrowBookMutation = useMutation({
    mutationFn: ({ bookId, userId }: { bookId: number; userId: number }) =>
      booksApi.borrow(bookId, { user_id: userId }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] });
      setBorrowingBookId(null);
      setBorrowUserId('');
    },
    onError: (error: any) => {
      console.error('Failed to borrow book:', error);
      alert(`Failed to borrow book: ${error.response?.data?.message || error.message}`);
    },
  });

  const returnBookMutation = useMutation({
    mutationFn: (bookId: number) => booksApi.return(bookId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['books'] });
    },
    onError: (error: any) => {
      console.error('Failed to return book:', error);
      alert(`Failed to return book: ${error.response?.data?.message || error.message}`);
    },
  });

  const handleCreateBook = (e: React.FormEvent) => {
    e.preventDefault();
    if (newBookTitle.trim() && newBookDivisionId !== '') {
      createBookMutation.mutate({
        title: newBookTitle.trim(),
        division_id: newBookDivisionId as number,
      });
    }
  };

  const handleBorrowBook = (bookId: number) => {
    if (borrowUserId !== '') {
      borrowBookMutation.mutate({ bookId, userId: borrowUserId as number });
    }
  };

  const getDivisionName = (divisionId: number) => {
    return divisions.find((d) => d.id === divisionId)?.name || 'Unknown';
  };

  const getUserName = (userId: number | null) => {
    if (!userId) return null;
    return users.find((u) => u.id === userId)?.name || 'Unknown';
  };

  if (booksLoading) {
    return (
      <div className="flex justify-center items-center py-12">
        <div className="text-gray-500">Loading books...</div>
      </div>
    );
  }

  if (booksError) {
    return (
      <div className="px-4 sm:px-6 lg:px-8">
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <p className="text-red-800">Failed to load books. Please check if the backend is running.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="px-4 sm:px-6 lg:px-8">
      <PageHeader
        title="Books"
        description="Manage library books, track borrowing and returns"
      />

      <div className="bg-white shadow-sm rounded-lg border border-gray-200 p-6 mb-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Add New Book</h2>
        <form onSubmit={handleCreateBook} className="space-y-4">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div>
              <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">
                Title
              </label>
              <input
                type="text"
                id="title"
                value={newBookTitle}
                onChange={(e) => setNewBookTitle(e.target.value)}
                placeholder="Enter book title"
                className="block w-full rounded-lg border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 px-4 py-2 border"
                required
              />
            </div>
            <div>
              <label htmlFor="division" className="block text-sm font-medium text-gray-700 mb-1">
                Division
              </label>
              <select
                id="division"
                value={newBookDivisionId}
                onChange={(e) => setNewBookDivisionId(Number(e.target.value))}
                className="block w-full rounded-lg border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 px-4 py-2 border"
                required
              >
                <option value="">Select a division</option>
                {divisions.map((division) => (
                  <option key={division.id} value={division.id}>
                    {division.name}
                  </option>
                ))}
              </select>
            </div>
          </div>
          <button
            type="submit"
            disabled={createBookMutation.isPending}
            className="inline-flex justify-center items-center rounded-lg border border-transparent bg-indigo-600 px-6 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {createBookMutation.isPending ? (
              <>
                <svg className="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Adding...
              </>
            ) : (
              'Add Book'
            )}
          </button>
        </form>
        {createBookMutation.isError && (
          <div className="mt-3 text-sm text-red-600">
            Failed to create book. Please try again.
          </div>
        )}
      </div>

      <div className="bg-white shadow-sm rounded-lg border border-gray-200 overflow-hidden">
        {books.length === 0 ? (
          <div className="text-center py-12">
            <h3 className="mt-2 text-sm font-medium text-gray-900">No books</h3>
            <p className="mt-1 text-sm text-gray-500">Get started by adding a new book to the library.</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    ID
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Title
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Division
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Status
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Actions
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {books.map((book) => (
                  <tr key={book.id} className="hover:bg-gray-50 transition-colors">
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                      {book.id}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {book.title}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-700">
                      {getDivisionName(book.division_id)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      {book.borrowed_by_user_id ? (
                        <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                          Borrowed by {getUserName(book.borrowed_by_user_id)}
                        </span>
                      ) : (
                        <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                          Available
                        </span>
                      )}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">
                      {book.borrowed_by_user_id ? (
                        <button
                          onClick={() => returnBookMutation.mutate(book.id)}
                          disabled={returnBookMutation.isPending}
                          className="text-indigo-600 hover:text-indigo-900 font-medium disabled:opacity-50 transition-colors"
                        >
                          Return
                        </button>
                      ) : (
                        <div className="flex items-center gap-2">
                          {borrowingBookId === book.id ? (
                            <>
                              <select
                                value={borrowUserId}
                                onChange={(e) => setBorrowUserId(Number(e.target.value))}
                                className="rounded-md border-gray-300 text-sm px-2 py-1 border focus:border-indigo-500 focus:ring-indigo-500"
                              >
                                <option value="">Select user</option>
                                {users.map((user) => (
                                  <option key={user.id} value={user.id}>
                                    {user.name}
                                  </option>
                                ))}
                              </select>
                              <button
                                onClick={() => handleBorrowBook(book.id)}
                                disabled={borrowUserId === '' || borrowBookMutation.isPending}
                                className="text-indigo-600 hover:text-indigo-900 font-medium disabled:opacity-50 transition-colors"
                              >
                                Confirm
                              </button>
                              <button
                                onClick={() => {
                                  setBorrowingBookId(null);
                                  setBorrowUserId('');
                                }}
                                className="text-gray-600 hover:text-gray-900 transition-colors"
                              >
                                Cancel
                              </button>
                            </>
                          ) : (
                            <button
                              onClick={() => setBorrowingBookId(book.id)}
                              className="text-indigo-600 hover:text-indigo-900 font-medium transition-colors"
                            >
                              Borrow
                            </button>
                          )}
                        </div>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
