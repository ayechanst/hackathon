import React from "react";

interface BreadcrumbsProps {
  breadcrumbList: string[];
}

const Breadcrumbs: React.FC<BreadcrumbsProps> = ({ breadcrumbList }) => {
  const breadcrumbArray = breadcrumbList;

  return (
    <>
      <div className="bg-gradient-to-r from-yellow-500 rounded-full text-white px-5">
        <div className="text-sm breadcrumbs">
          <ul>
            {breadcrumbArray.map(crumb => (
              <li key={crumb}>
                <a>{crumb}</a>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </>
  );
};
export default Breadcrumbs;
