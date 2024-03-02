import React from "react";

interface BreadcrumbsProps {
  breadcrumbList: string[];
}

const Breadcrumbs: React.FC<BreadcrumbsProps> = ({ breadcrumbList }) => {
  const breadcrumbArray = breadcrumbList;

  return (
    <>
      <div className=" bg-yellow-500 rounded-full text-primary px-5 justify-end">
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
