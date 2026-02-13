# Office Library System - Frontend

Modern frontend application for the Office Library Management System.

## Tech Stack

- **Framework**: React 18 with TypeScript
- **Build Tool**: Vite
- **Styling**: Tailwind CSS
- **Data Fetching**: TanStack Query (React Query)
- **Routing**: React Router v6
- **HTTP Client**: Axios

## Prerequisites

- Node.js 18+ and npm

## Getting Started

### Install Dependencies

```bash
npm install
```

### Environment Configuration

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` to configure the API endpoint:

```env
VITE_API_URL=http://localhost:8080
```

### Development Server

```bash
npm run dev
```

The application will be available at `http://localhost:5173`

### Build for Production

```bash
npm run build
```

The build output will be in the `dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## Features

### Pages

- **Books**: View all books, add new books, borrow and return books
- **Divisions**: Manage organizational divisions
- **Users**: Manage users and their division assignments

### Key Features

- Real-time data updates using TanStack Query
- Responsive design with Tailwind CSS
- Type-safe API integration with TypeScript
- Optimistic UI updates
- Loading and error states

## Project Structure

```
frontend/
├── src/
│   ├── lib/
│   │   └── api.ts           # API client and types
│   ├── pages/
│   │   ├── Books.tsx        # Books management page
│   │   ├── Divisions.tsx    # Divisions management page
│   │   └── Users.tsx        # Users management page
│   ├── App.tsx              # Main app component with routing
│   ├── main.tsx             # Application entry point
│   └── index.css            # Global styles with Tailwind
├── .env                     # Environment configuration
└── vite.config.ts           # Vite configuration
```

## API Integration

The frontend connects to the backend REST API. See `src/lib/api.ts` for all available endpoints:

- `POST /divisions` - Create division
- `POST /users` - Create user
- `POST /books` - Create book
- `GET /books` - List books
- `GET /books/{id}` - Get book details
- `POST /books/{id}/borrow` - Borrow book
- `POST /books/{id}/return` - Return book
